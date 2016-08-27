#![feature(box_syntax, box_patterns, slice_patterns, plugin)]
#![plugin(rabbot_plugin)]

extern crate rabbot;

mod ast;
mod grammar;
mod interpreter;

pub fn compile(input: String) {
    let abt = grammar::parse_Block(input.as_str()).unwrap();
    println!("{:?}", abt);
    println!("{:?}", interpreter::eval(abt));
}

#[cfg(test)]
mod tests {
    use super::compile;

    #[test]
    fn it_works() {
        compile("let x = fn y => { y + 1 }; x 1".to_string());
    }
}
