#![feature(plugin, box_syntax, test, concat_idents)]
#![plugin(lia_plugin)]

#[macro_use] extern crate lia_runtime;
extern crate test;

// mod matrix;
mod lists;

use lia_runtime::*;

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
        (function() { x += 1; })();
        return x;
    }

    function fib_test(n) {
        if (n <= 1) { return n; }
        return @fib_test(n - 1) + @fib_test(n - 2);
    }

    function nested_object_test() {
        var x = {foo: {bar: 2}};
        x.foo.bar = 3;
        return x.foo.bar;
    }

    function while_test() {
        var x = 0;
        while (x < 10) {
            x += 1;
        }
        return x;
    }

    function for_test() {
        for (var x = 0; x < 10; x += 1) {}
        return x;
    }

    function foreach_test() {
        var x = {foo: 1, bar: 2};
        var z = 0;
        for (var y : x) {
            z += x[y];
        }
        return z;
    }

    function self_ref_test() {
        var x = {
            y: function() {
                this.z = 5;
            },
            z: 0
        };
        x.y();
        return x.z;
    }

    function if_test() {
        var x = 0;
        var y = {};
        if (0) { x += 1; }
        if (true) { x += 1; }
        if (y.foo) { x += 1; }
        return x;
    }

    function if_else_test() {
        var x = 0;
        if (0) { x = 1; }
        else { x = 2; }
        return x;
    }

    function double_add_test() {
        var x = 1;
        x += x;
        return x;
    }

    function concat_test() {
        var x = "foo";
        x += "bar";
        return x;
    }

    function str_eq_test() {
        var x = 0;
        if ("foo" == "foo") { x += 1; }
        if ("bar" == "foo") { x += 1; }
        return x;
    }
}

fn _lia_external_fun(args: Vec<LiaPtr>) -> LiaPtr {
    cast!(let num: i32 = args[1].clone());
    return alloc_number(num + 1);
}

// TODO: auto-generate function name somehow?
macro_rules! gen_test {
    ($test:ident, $fun:ident, $ty:ty, $val:expr) => {
        #[test]
        fn $test () {
            cast!(let result: $ty = call!($fun()));
            assert_eq!(result, $val);
        }
    }
}

gen_test!(lia_add_test, add_test, i32, 3);
gen_test!(lia_string_test, string_test, String, "Hello world!");
gen_test!(lia_extern_test, extern_test, i32, 4);
gen_test!(lia_by_ref_test, by_ref_test, i32, 1);
gen_test!(lia_by_val_test, by_val_test, i32, 3);
gen_test!(lia_closure_test, closure_test, i32, 1);
gen_test!(lia_nested_object_test, nested_object_test, i32, 3);
gen_test!(lia_while_test, while_test, i32, 10);
gen_test!(lia_for_test, for_test, i32, 10);
gen_test!(lia_foreach_test, foreach_test, i32, 3);
gen_test!(lia_self_ref_test, self_ref_test, i32, 5);
gen_test!(lia_if_test, if_test, i32, 1);
gen_test!(lia_if_else_test, if_else_test, i32, 2);
gen_test!(lia_double_add_test, double_add_test, i32, 2);
gen_test!(lia_concat_test, concat_test, String, "foobar");
gen_test!(lia_str_eq_test, str_eq_test, i32, 1);

#[test]
fn lia_fib_test() {
    cast!(let num: i32 = call!(fib_test(alloc_number(10))));
    assert!(num == 55);
}

// TODO: only run this when user does cargo bench
// use test::Bencher;
// #[bench]
// fn lia_fib_bench(b: &mut Bencher) {
//     b.iter(|| call!(fib_test(alloc_number(30))));
// }
