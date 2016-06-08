#![feature(plugin, box_syntax)]
#![plugin(lia_plugin)]

#[macro_use]
extern crate lia;

use lia::runtime::*;

lia! {
    function foo() {
        var x = [1, 2, 3];
        return 0;
    }
}

#[test]
fn macro_test() {
    let result: LiaAny = call!(foo());
    cast!(let num: i32 = result);
    println!("result {}", num);
}
