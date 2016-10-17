#![feature(box_syntax, box_patterns, slice_patterns, plugin, rustc_private,
           quote, question_mark, try_from)]
#![plugin(rabbot_plugin)]

#[macro_use] extern crate rabbot;
#[macro_use] extern crate lia_jit;
extern crate rustlex_codegen as rustlex;
extern crate syntax;
extern crate llvm;

use std::io::BufReader;
use lia_jit::{JitOptions, get_sysroot};

use mark::Marked;
use token::Token;
use lexer::Lexer;
use ast::term::Term;
use grammar::{parse_Toplevel as parse_toplevel};
use interpreter::EvalState;

mod token;
mod mark;
mod lexer;
mod ast;
mod grammar;
mod pprint;
mod typecheck;
mod interpreter;

macro_rules! make_state {
    ($state:ident) => {
        let sess = syntax::parse::ParseSess::new();
        let cfg = vec![];
        let ecfg = syntax::ext::expand::ExpansionConfig::default("_".to_string());
        let mut loader = syntax::ext::base::DummyMacroLoader;
        let cx = syntax::ext::base::ExtCtxt::new(&sess, cfg, ecfg, &mut loader);
        make_jit!(jit, JitOptions { sysroot: get_sysroot() });
        let mut $state = EvalState { cx: cx, jit: jit };
    }
}

pub fn compile(input: String) -> Term {
    let input = BufReader::new(input.as_bytes());
    let lexer = Lexer::new(input);
    let tokens = lexer.collect::<Vec<Marked<Token>>>();
    // println!("{:?}", tokens);
    let abt = parse_toplevel(tokens).unwrap();
    if let Err(err) = typecheck::infer(abt.clone()) {
        panic!("{}", err)
    }

    make_state!(state);

    // Compiler complains if we use the canonical return form. ¯\_(ツ)_/¯
    interpreter::eval(&mut state, abt)
}

#[cfg(test)]
mod tests {
    use super::compile;

    use ast::term::{View, out};

    #[test]
    fn call_function() {
        let src = "
let x = fn y => { y + 1 };
(x 1)";
        bind!(View::Number{n} = out(compile(src.to_string())).val);
        assert_eq!(n, 2);
    }

    #[test]
    fn quote() {
        let result = out(compile("
let n = 3;
let incr: i32 -> i32 = $rs {
  fn incr_n(x: i32) -> i32 { x + $n }
};
(incr 1)".to_string()));
        bind!(View::Number{n} = result.val);
        assert_eq!(n, 4);
    }

    #[test]
    fn preserve_polymorphism() {
        let src = r#"let x = fn y => { 1 }; (x 1); (x "foo"); x"#;
        out(compile(src.to_string()));
    }

    #[test]
    fn type_alias() {
        let src = "type X = i32; let n: X = 3; n";
        out(compile(src.to_string()));
    }

    #[test]
    fn product() {
        let src = "type Foo = { x: i32 }; let x = { x: 3 }; x.x";
        out(compile(src.to_string()));
    }
}
