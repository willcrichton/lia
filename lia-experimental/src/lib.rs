#![feature(box_syntax, box_patterns, slice_patterns, plugin, rustc_private, quote, question_mark)]
#![plugin(rabbot_plugin)]

#[macro_use] extern crate rabbot;
#[macro_use] extern crate lia_jit;
extern crate rustlex_codegen as rustlex;
extern crate syntax;
extern crate llvm;

use std::io::BufReader;
use lia_jit::{JitOptions, get_sysroot};
use llvm::ExecutionEngine;

use token::Token;
use lexer::Lexer;
use ast::term::Term;
use grammar::{parse_Block as parse_block};
use interpreter::EvalState;

mod token;
mod lexer;
mod ast;
mod grammar;
mod typecheck;
mod interpreter;

macro_rules! make_state {
    ($state:ident) => {
        let sess = syntax::parse::ParseSess::new();
        let cfg = vec![];
        let ecfg = syntax::ext::expand::ExpansionConfig::default("_".to_string());
        let mut loader = syntax::ext::base::DummyMacroLoader;
        let mut cx = syntax::ext::base::ExtCtxt::new(&sess, cfg, ecfg, &mut loader);
        make_jit!(jit, JitOptions { sysroot: get_sysroot() });
        let mut $state = EvalState { cx: cx, jit: jit };
    }
}

pub fn compile(input: String) -> Term {
    let input = BufReader::new(input.as_bytes());
    let lexer = Lexer::new(input);
    let tokens = lexer.collect::<Vec<Token>>();
    // println!("{:?}", tokens);
    let abt = parse_block(tokens).unwrap();
    println!("{:?}", typecheck::infer(abt.clone()));
    panic!()

    // make_state!(state);

    // // Compiler complains if we use the canonical return form. ¯\_(ツ)_/¯
    // let val = interpreter::eval(&mut state, abt);
    // val
}

#[cfg(test)]
mod tests {
    use super::compile;

    use ast::term::{View, out};

    #[test]
    fn simple() {
        let src = "
let x = fn y => { y + 1 };
(x 1)";
        match out(compile(src.to_string())) {
            View::Number(n) => assert_eq!(n, 2),
            _ => panic!()
        }
    }

    #[test]
    fn quote() {
        let result = out(compile("
let n = 3;
let incr = $rs {
  fn incr_n(x: i32) -> i32 { x + $n }
};
(incr 1)".to_string()));
        bind!(View::Number{n} = result);
        assert_eq!(n, 4);
    }


    #[test]
    fn fun() {
        let src = r#"let x = fn y => { 1 }; (x 1); (x "foo"); x"#;
        out(compile(src.to_string()));
    }
}
