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
    //println!("{:?}", abt);
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

    macro_rules! test_pass {
        ($name:ident, $src:expr) => {
            #[test]
            fn $name () {
                compile($src.to_string());
            }
        };
        ($name:ident, $src:expr, $expected:expr) => {
            #[test]
            fn $name () {
                bind!(View::Number{n} = out(compile($src.to_string())).val);
                assert_eq!(n, $expected);
            }
        };
    }

    macro_rules! test_fail {
        ($name:ident, $src:expr) => {
            #[test]
            #[should_panic]
            fn $name () {
                compile($src.to_string());
            }
        }
    }

    test_pass!(
        call_function,
        "let x = fn y => { y + 1 }; (x 0)",
        1);

    test_pass!(
        quote,
        "
let n = 0;
let incr: i32 -> i32 = $rs {
  fn incr_n(x: i32) -> i32 { x + $n }
};
(incr 1)",
        1);

    test_pass!(
        preserve_polymorphism,
        r#"let x = fn y => { 1 }; (x 1); (x "foo"); x"#);

    test_pass!(
        type_alias,
        "type X = i32; let n: X = 3; n");

    test_pass!(
        product_type,
        "type Foo = { y: i32 }; let x = { y: 3 }; x.y",
        3);

    test_fail!(
        product_no_type_decl,
        "let x = { y: 3 }; x.y");

}
