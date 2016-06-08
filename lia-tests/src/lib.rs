#![feature(plugin, box_syntax, test)]
#![plugin(lia_plugin)]

#[macro_use]
extern crate lia;
extern crate test;

mod matrix;

use lia::runtime::*;
use test::Bencher;

lia! {
    function add_test() {
        return 1 + 2;
    }

    function string_test() {
        return "Hello world!";
    }

    function extern_test() {
        return @external_fun(3);
    }

    function by_ref_test() {
        var x = {"foo": 0};
        var y = x;
        y["foo"] = 1;
        return x["foo"];
    }

    function by_val_test() {
        var x = 3;
        var y = x;
        y = 2;
        return x;
    }

    function closure_test() {
        var x = 0;
        (function() { x = x + 1; })();
        return x;
    }

    function fib_test(n) {
        var fib_fn = function(n) {
            if (n == 0) { return 0; }
            if (n == 1) { return 1; }
            return fib_fn(n - 1) + fib_fn(n - 2);
        };
        return fib_fn(n);
    }

    function nested_object_test() {
        var x = {foo: {bar: 3}};
        return x.foo.bar;
    }
}

fn _lia_external_fun(args: Vec<LiaAny>) -> LiaAny {
    cast!(let num: i32 = args[0].clone());
    return alloc(*num + 1);
}

#[test]
fn lia_add_test() {
    cast!(let num: i32 = call!(add_test()));
    assert!(*num == 3);
}

#[test]
fn lia_string_test() {
    cast!(let string: String = call!(string_test()));
    assert!(string == "Hello world!");
}

#[test]
fn lia_extern_test() {
    cast!(let num: i32 = call!(extern_test()));
    assert!(*num == 4);
}

#[test]
fn lia_by_ref_test() {
    cast!(let num: i32 = call!(by_ref_test()));
    assert!(*num == 1);
}

#[test]
fn lia_by_val_test() {
    cast!(let num: i32 = call!(by_val_test()));
    assert!(*num == 3);
}

#[test]
fn lia_closure_test() {
    cast!(let num: i32 = call!(closure_test()));
    assert!(*num == 1);
}

#[test]
fn lia_fib_test() {
    cast!(let num: i32 = call!(fib_test(10)));
    assert!(*num == 55);
}

// TODO: only run this when user does cargo bench?
// #[bench]
// fn lia_fib_bench(b: &mut Bencher) {
//     b.iter(|| call!(fib_test(30)));
// }

#[test]
fn lia_nested_object_test() {
    cast!(let num: i32 = call!(nested_object_test()));
    assert!(*num == 3);
}
