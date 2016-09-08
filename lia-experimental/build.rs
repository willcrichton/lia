extern crate lalrpop;
extern crate syntex;
extern crate rustlex_codegen;

use std::env;
use std::path::Path;

fn main () {
    lalrpop::process_root().unwrap();

    let mut registry = syntex::Registry::new();
    rustlex_codegen::plugin_registrar(&mut registry);
    let src = Path::new("src/lexer.in.rs");
    let dst = Path::new("src/lexer.rs");
    println!("{:?}, {:?}", src, dst);
    registry.expand("", &src, &dst).unwrap();
}
