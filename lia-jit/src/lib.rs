#![feature(plugin, rustc_private, box_syntax)]
#![plugin(lia_plugin)]

extern crate rustc;
extern crate rustc_driver;
extern crate rustc_lint;
extern crate rustc_metadata;
extern crate rustc_llvm;
extern crate rustc_resolve;
#[macro_use] extern crate syntax;
extern crate getopts;

extern crate llvm;
extern crate llvm_sys;
#[macro_use] extern crate lia;

use rustc_driver::{CompilerCalls, Compilation};
use rustc_driver::driver::CompileController;
use rustc::session::Session;
use rustc::middle::cstore::LinkagePreference;
use std::ffi::{CString, CStr};
use syntax::codemap::FileLoader;
use std::io;
use std::path::{PathBuf, Path};
// use rustc_llvm;
// use getopts;
// use llvm;
// use llvm_sys;
use std::rc::Rc;
use std::cell::RefCell;

struct JitInput {
    input: String
}

impl JitInput {
    pub fn new(input: String) -> JitInput {
        JitInput {
            input: input
        }
    }
}

impl FileLoader for JitInput {
    fn file_exists(&self, _: &Path) -> bool { true }
    fn abs_path(&self, _: &Path) -> Option<PathBuf> { None }
    fn read_file(&self, _: &Path) -> io::Result<String> { Ok(self.input.clone()) }
}

struct JitState<'a> {
    ctx: &'a llvm::CSemiBox<'a, llvm::Context>,
    engine: llvm::JitEngine<'a>,
    main_module: llvm::CSemiBox<'a, llvm::Module>,
    other_modules: Vec<llvm::CSemiBox<'a, llvm::Module>>,
    eval: bool,
    name: String,
    anon_count: u32,
}

static EVAL_FN: &'static str = "_jit_eval";

impl<'a> JitState<'a> {
    fn process_llvm(&self, llmod: &llvm::CSemiBox<'a, llvm::Module>) {
        use llvm::*;
        let fun = llmod.get_function(self.name.as_str())
            .expect(format!("LLVM global `{}` missing", self.name).as_str());
        self.main_module.add_global_variable(self.name.as_str(), &***fun);
        if self.eval {
            let fun = self.engine.find_function(EVAL_FN).unwrap();
            self.engine.with_function(fun, |fun: extern fn(())| fun(()));
        }
    }
}

pub struct JitOptions {
    pub sysroot: String
}

pub struct Jit<'a> {
    state: Rc<RefCell<JitState<'a>>>,
    opts: JitOptions,
}

impl<'a> Jit<'a> {
    pub fn new(ctx: &'a llvm::CSemiBox<'a, llvm::Context>, opts: JitOptions) -> Jit<'a> {
        use llvm::{Module, JitEngine, ExecutionEngine};
        let module = Module::new("_jit_main", &ctx);
        let engine = JitEngine::new(&module, llvm::JitOptions {opt_level: 0})
            .expect("Jit not initialized");
        Jit {
            opts: opts,
            state: Rc::new(RefCell::new(JitState {
                ctx: ctx,
                engine: engine,
                main_module: module,
                other_modules: vec![],
                eval: false,
                name: "".to_string(),
                anon_count: 0u32,
            }))
        }
    }

    pub fn run(&mut self, input: String) {
        use rustc_driver;
        use syntax::parse;

        let crate_name = "jit".to_string();
        let sess = parse::ParseSess::new();

        let (input, eval, name) = match parse::parse_item_from_source_str(
            crate_name.clone(), input.clone(), vec![], &sess)
        {
            Ok(Some(item)) => {
                (input, false, format!("{}", item.unwrap().ident))
            },
            err => {
                if let Err(mut err) = err {
                    err.cancel();
                }

                match parse::parse_expr_from_source_str(
                    crate_name.clone(), input.clone(), vec![], &sess)
                {
                    Ok(_) => {
                        let mut state = self.state.borrow_mut();
                        let anon_fn = format!("{}_{}", EVAL_FN, state.anon_count);
                        state.anon_count += 1;
                        (format!(r#"#[no_mangle] fn {} () {{ println!("{{:?}}", {}) }}"#, EVAL_FN, input),
                         true,
                         anon_fn)
                    },
                    Err(mut err) => {
                        err.cancel();
                        println!("Input was neither expression nor item");
                        return;
                    }
                }
            }
        };

        {
            let mut state = self.state.borrow_mut();
            state.eval = eval;
            state.name = name;
        }

        let input = JitInput::new(input);
        let args: Vec<String> =
            format!(
                "_ {} --sysroot {} --crate-type dylib --cap-lints allow",
                crate_name,
                self.opts.sysroot)
            .split(' ').map(|s| s.to_string()).collect();
        rustc_driver::run_compiler_with_file_loader(&args, self, box input);
    }
}

/// Returns last error from LLVM wrapper code.
fn llvm_error() -> String {
    String::from_utf8_lossy(
        unsafe { CStr::from_ptr(rustc_llvm::LLVMRustGetLastError()).to_bytes() })
        .into_owned()
}

impl<'a> CompilerCalls<'a> for Jit<'a> {
    fn build_controller(&mut self,
                        _: &Session,
                        _: &getopts::Matches)
                        -> CompileController<'a> {
        let mut cc: CompileController<'a> = CompileController::basic();
        cc.after_llvm.stop = Compilation::Stop;
        cc.after_llvm.run_callback_on_error = true;
        let jit_state = self.state.clone();
        cc.after_llvm.callback = Box::new(move |state| {
            state.session.abort_if_errors();
            let trans = state.trans.unwrap();
            assert_eq!(trans.modules.len(), 1);

            let rs_llmod = trans.modules[0].llmod;
            assert!(!rs_llmod.is_null());

            let crates = state.session.cstore.used_crates(LinkagePreference::RequireDynamic);

            // Collect crates used in the session. Reverse order finds dependencies first.
            let deps: Vec<PathBuf> =
                crates.into_iter().rev().filter_map(|(_, p)| p).collect();
            for path in deps {
                let s = match path.as_os_str().to_str() {
                    Some(s) => s,
                    None => panic!(
                        "Could not convert crate path to UTF-8 string: {:?}", path)
                };
                let cs = CString::new(s).unwrap();
                let res = unsafe { rustc_llvm::LLVMRustLoadDynamicLibrary(cs.as_ptr()) };
                if res == 0 {
                    panic!("Failed to load crate {:?}: {}", path.display(), llvm_error());
                }
            }

            let llmod: &mut llvm::Module =
                (rs_llmod as llvm_sys::prelude::LLVMModuleRef).into();
            let llmod = llmod.clone();
            //llmod.verify().expect("Module invalid");

            let mut state = jit_state.borrow_mut();
            state.process_llvm(&llmod);
            state.other_modules.push(llmod);
        });
        cc
    }
}

#[macro_export]
macro_rules! make_context {
    ($id:ident) => {
        let $id = {
            use llvm::Context;
            Context::new()
        };
        let $id = $id.as_semi();
    }
}

#[cfg(test)]
mod test {
    static SYSROOT: &'static str =
        "/Users/will/.multirust/toolchains/nightly-x86_64-apple-darwin";

    use super::*;

    macro_rules! make_test {
        ($fun:ident, $s:expr) => {
            #[test]
            fn $fun() {
                make_context!(ctx);
                let mut jit = Jit::new(ctx, JitOptions { sysroot: SYSROOT.to_string() });
                let input = $s.to_string();
                jit.run(input);
            }
        }
    }

    make_test!(compile_test, r#"#[no_mangle] pub fn test_add(a: i32, b: i32) -> i32 { a + b }"#);
    make_test!(expr_test, "1 + 2");
}
