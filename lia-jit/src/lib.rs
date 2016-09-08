#![feature(plugin, rustc_private, box_syntax)]

extern crate rustc;
extern crate rustc_driver;
extern crate rustc_lint;
extern crate rustc_metadata;
extern crate rustc_llvm;
extern crate rustc_resolve;
extern crate rustc_trans;
#[macro_use] extern crate syntax;
extern crate getopts;

extern crate llvm;
extern crate llvm_sys;

use rustc_trans::ModuleSource;
use rustc_driver::{CompilerCalls, Compilation};
use rustc_driver::driver::CompileController;
use rustc::session::Session;
use rustc::middle::cstore::LinkagePreference;
use syntax::codemap::FileLoader;
use syntax::print::pprust;
use syntax::ast::ItemKind;
use syntax::parse::token::str_to_ident;
use std::ffi::CString;
use std::io;
use std::path::{PathBuf, Path};
use std::rc::Rc;
use std::cell::RefCell;
use std::any::Any;
use std::convert::From;
use llvm::{ExecutionEngine, JitEngine};
use llvm_sys::execution_engine::{LLVMExecutionEngineRef as LLVMEngine};


#[derive(Clone)]
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

#[allow(dead_code)]
struct JitState<'a, Eng>
    where Eng: ExecutionEngine<'a>, LLVMEngine: From<&'a Eng>
{
    engine: llvm::CSemiBox<'a, Eng>,
    other_modules: Vec<llvm::CSemiBox<'a, llvm::Module>>,
    name: String,
    anon_count: u32,
    return_slot: Box<Any>,
    funs: String,
}

impl<'a> JitState<'a, JitEngine>
{
    fn gen_fn(&self, llmod: &llvm::CSemiBox<'a, llvm::Module>)
              -> Rc<extern fn(()) -> i32>
    {
        self.engine.add_module(llmod);
        let fun = self.engine.find_function(self.name.as_str()).expect("Function not found");
        let fun = unsafe { self.engine.get_function(fun) };
        Rc::new(fun)
    }
}

pub struct JitOptions {
    pub sysroot: String
}

pub struct Jit<'a, Eng>
    where Eng: ExecutionEngine<'a>, LLVMEngine: From<&'a Eng>
{
    state: Rc<RefCell<JitState<'a, Eng>>>,
    opts: JitOptions,
}

impl<'a> Jit<'a, JitEngine> {
    pub fn new(engine: llvm::CSemiBox<'a, JitEngine>,
               opts: JitOptions)
               -> Jit<'a, JitEngine>
    {
        Jit {
            opts: opts,
            state: Rc::new(RefCell::new(JitState {
                engine: engine,
                other_modules: vec![],
                name: "".to_string(),
                funs: "".to_string(),
                anon_count: 0u32,
                return_slot: box 0
            }))
        }
    }

    pub fn gen_fun(&mut self, input: String) -> Result<Rc<extern fn(()) -> i32>, String>
    {
        use rustc_driver;
        use syntax::parse;

        let crate_name = "jit".to_string();
        let sess = parse::ParseSess::new();

        let (input, name, decl) = match parse::parse_item_from_source_str(
            crate_name.clone(), input.clone(), vec![], &sess)
        {
            Ok(Some(item)) => {
                let item = item.unwrap();
                let name = item.ident;
                let (input, decl) = match item.node {
                    ItemKind::Fn(decl, unsafety, constness, _, generics, body) => {
                        let name_u = str_to_ident(format!("_{}", name.name.as_str()).as_str());
                        let extern_s = pprust::fun_to_string(
                            &decl.clone().unwrap(), unsafety, constness.node,
                            name_u.clone(), &generics);
                        let decl_s = pprust::fun_to_string(
                            &decl.clone().unwrap(), unsafety, constness.node,
                            name.clone(), &generics);
                        let args = decl.inputs.iter()
                            .map(|arg| pprust::pat_to_string(&arg.pat.clone().unwrap()))
                            .collect::<Vec<String>>()
                            .join(",");
                        let block = pprust::block_to_string(&body.unwrap());
                        (format!("#[no_mangle] {} {{ {} }} \
                                  #[no_mangle] {} {{ {}({}) }}",
                                 extern_s, block, decl_s, name_u, args),
                         format!("extern {{ {}; }} \
                                  #[no_mangle] {} {{ unsafe {{ {}({}) }} }}",
                                 extern_s, decl_s, name_u, args))
                    }
                    _ => return Err("Not a function".to_string())
                };
                (input, name, decl)
            }
            Err(mut err) => {
                err.cancel();
                return Err(err.message.clone());
            },
            Ok(None) => { return Err("Bad parse".to_string()); }
        };

        let input = {
            let mut state = self.state.borrow_mut();
            let input = format!("{}\n#[no_mangle] {}", state.funs, input);
            state.name = name.name.as_str().to_string();
            input
        };

        let jit_input = JitInput::new(input.clone());
        let args: Vec<String> =
            format!(
                "_ {} --sysroot {} --crate-type dylib --cap-lints allow",
                crate_name,
                self.opts.sysroot)
            .split(' ').map(|s| s.to_string()).collect();

        if let (Err(n), _) =
            rustc_driver::run_compiler_with_file_loader(&args, self, box jit_input)
        {
            return Err(format!("Compilation error {}", n));
        };
        rustc_driver::driver::reset_thread_local_state();

        {
            let mut state = self.state.borrow_mut();
            state.funs = format!("{}\n{}", state.funs, decl);
            match state.return_slot.downcast_ref::<Rc<extern fn(()) -> i32>>() {
                Some(f) => Ok(f.clone()),
                None => Err("Incorrect type".to_string())
            }
        }
    }
}

impl<'a> CompilerCalls<'a> for Jit<'a, JitEngine> {
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

            let rs_llmod = match trans.modules[0].source {
                ModuleSource::Translated(llmod) => llmod.llmod,
                ModuleSource::Preexisting(_) => unreachable!()
            };
            assert!(!rs_llmod.is_null());

            //unsafe { rustc_llvm::LLVMDumpModule(rs_llmod) };

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
                let res = unsafe { llvm_sys::support::LLVMLoadLibraryPermanently(cs.as_ptr()) };
                if res != 0 {
                    panic!("Failed to load crate {:?}", path.display());
                }
            }

            let llmod: &'a llvm::Module =
                (rs_llmod as llvm_sys::prelude::LLVMModuleRef).into();
            let llmod = llmod.clone();
            llmod.verify().expect("Module invalid");

            let mut state = jit_state.borrow_mut();
            let fun = state.gen_fn(&llmod);
            state.return_slot = box fun;

            state.other_modules.push(llmod);

        });
        cc
    }
}

#[macro_export]
macro_rules! make_jit {
    ($jit:ident, $opts:expr) => {
        let _jit_ctx = ::llvm::Context::new();
        let _jit_ctx = _jit_ctx.as_semi();
        let module = ::llvm::Module::new("_jit_main", &_jit_ctx);
        let engine = {
            use llvm::ExecutionEngine;
            ::llvm::JitEngine::new(&module, ::llvm::JitOptions {opt_level: 0})
                .expect("Jit not initialized")
        };
        let $jit = ::lia_jit::Jit::new(engine, $opts);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static SYSROOT: &'static str =
        "/Users/will/.multirust/toolchains/nightly-x86_64-apple-darwin";

    macro_rules! make_test {
        ($fun:ident, $s:expr) => {
            #[test]
            fn $fun() {
                let _jit_ctx = {
                    use llvm::Context;
                    Context::new()
                };
                let _jit_ctx = _jit_ctx.as_semi();
                let module = ::llvm::Module::new("_jit_main", &_jit_ctx);
                let mut jit =
                    Jit::new(_jit_ctx, &module, JitOptions { sysroot: SYSROOT.to_string() });
                let input = $s.to_string();
                jit.gen_fun(input);
            }
        }
    }

    //make_test!(compile_test, r#"#[no_mangle] pub fn test_add(a: i32, b: i32) -> i32 { a + b }"#);
    //make_test!(expr_test, "1 + 2");
    // make_test!(print_test, "{println!(\"hello world\");}");
}
