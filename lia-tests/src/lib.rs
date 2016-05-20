#![feature(plugin)]
#![plugin(lia_plugin)]

#[macro_use]
extern crate lia;

lia! {
    function foo() {
        var x = 1 + 2;
        return x;
    }
}

#[test]
fn macro_test() {
    let result = foo();
    println!("Result: {}", cast!(result, i32));
}
