#![feature(rustc_private, question_mark)]

extern crate syntax;
extern crate getopts;
extern crate llvm;
#[macro_use] extern crate lia_jit;

use llvm::JitEngine;
use lia_jit::Jit;

static EVAL_FN: &'static str = "_jit_eval";

struct State<'a> {
    jit: Jit<'a, JitEngine>,
    anon_count: i32
}

fn run<'a>(st: &mut State<'a>, input: String) -> Result<(), String> {
    use syntax::parse;

    let sess = parse::ParseSess::new();
    let crate_name = "jit".to_string();

    let input = match parse::parse_item_from_source_str(
        crate_name.clone(), input.clone(), vec![], &sess)
    {
        Ok(Some(_)) => input,
        err => {
            if let Err(mut err) = err {
                err.cancel();
            }

            match parse::parse_expr_from_source_str(
                crate_name.clone(), input.clone(), vec![], &sess)
            {
                Ok(_) => {
                    let anon_fn = format!("{}_{}", EVAL_FN, st.anon_count);
                    st.anon_count += 1;
                    format!(r#"fn {} () {{ println!("{{:?}}", {{ {} }}) }}"#, anon_fn, input)
                },
                Err(mut err) => {
                    err.cancel();
                    return Err("Input was neither expression nor item".to_string())
                }
            }
        }
    };

    let fun = st.jit.gen_fun::<(),()>(input)?;
    fun(());

    Ok(())
}

fn repl(matches: getopts::Matches) {
    use std::io::{stdin, stdout, Write};
    use lia_jit::{JitOptions, get_sysroot};

    if !matches.free.is_empty() {
        panic!("Need to handle input files");
    }

    make_jit!(jit, JitOptions { sysroot: get_sysroot() });
    let mut st = State { jit: jit, anon_count: 0 };

    loop {
        let mut line = String::new();
        print!(">>> "); stdout().flush().unwrap();
        stdin().read_line(&mut line).expect("Failed to read line");
        match run(&mut st, line) {
            Ok(_) => (),
            Err(s) => println!("{}", s)
        }
    }
}

fn print_usage(program: &str, opts: &[getopts::OptGroup]) {
    let brief = format!("Usage: {} [FILE] [OPTIONS]", program);
    print!("{}", getopts::usage(&brief, opts));
}

fn main() {
    use getopts::{optflag, getopts};
    use std::env;

    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let opts = &[
        optflag("h", "help", "print this help menu")
    ];
    let matches = match getopts(&args[1..], opts) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    repl(matches);
}
