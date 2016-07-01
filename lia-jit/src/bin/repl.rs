#![feature(rustc_private)]

extern crate getopts;
extern crate llvm;
#[macro_use] extern crate lia_jit;

fn repl(matches: getopts::Matches) {
    use std::io::{stdin, stdout, Write};
    use lia_jit::{Jit, JitOptions};

    if !matches.free.is_empty() {
        panic!("Need to handle input files");
    }

    let sysroot = "/Users/will/.multirust/toolchains/nightly-x86_64-apple-darwin".to_string();
    make_context!(ctx);
    let mut jit = Jit::new(ctx, JitOptions { sysroot: sysroot });

    loop {
        let mut line = String::new();
        print!(">>> "); stdout().flush().unwrap();
        stdin().read_line(&mut line).expect("Failed to read line");
        jit.run(line);
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
