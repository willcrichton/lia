#![feature(plugin, rustc_private, box_syntax)]

extern crate rustc;
extern crate rustc_driver;
extern crate rustc_llvm;
#[macro_use] extern crate syntax;
extern crate getopts;

use rustc_driver::{CompilerCalls, Compilation};
use rustc_driver::driver::CompileController;
use rustc::session::Session;
use syntax::codemap::FileLoader;
use std::io;
use std::path::{PathBuf, Path};
use rustc::util::common::path2cstr;

struct JitLoader;

impl FileLoader for JitLoader {
    fn file_exists(&self, _: &Path) -> bool { true }
    fn abs_path(&self, _: &Path) -> Option<PathBuf> { None }
    fn read_file(&self, _: &Path) -> io::Result<String> {
        Ok(r#"
#[no_mangle]
pub fn test_add(a: i32, b: i32) -> i32 { a + b }
"#.to_string())
    }
}

#[derive(Copy, Clone)]
struct JitCalls;

impl<'a> CompilerCalls<'a> for JitCalls {
    fn build_controller(&mut self,
                        _: &Session,
                        _: &getopts::Matches)
                        -> CompileController<'a> {
        let mut cc = CompileController::basic();
        cc.after_llvm.stop = Compilation::Stop;
        cc.after_llvm.run_callback_on_error = true;
        cc.after_llvm.callback = Box::new(|state| {
            state.session.abort_if_errors();
            let trans = state.trans.unwrap();
            assert_eq!(trans.modules.len(), 1);
            let rs_llmod = trans.modules[0].llmod;
            assert!(!rs_llmod.is_null());

            unsafe {
                let cpm = rustc_llvm::LLVMCreatePassManager();
                let path = path2cstr(Path::new("test.ll"));
                rustc_llvm::LLVMRustPrintModule(cpm, rs_llmod, path.as_ptr());
            }
        });
        cc
    }
}

fn main() {
    use rustc_driver;
    let args: Vec<String> =
        "_ _ --sysroot /Users/will/.multirust/toolchains/nightly-x86_64-apple-darwin --crate-type dylib"
        .split(' ').map(|s| s.to_string()).collect();

    let (result, _) = rustc_driver::run_compiler_with_file_loader(
        &args, &mut JitCalls, box JitLoader);
    if let Err(n) = result {
        println!("Error {}", n);
    }
}
