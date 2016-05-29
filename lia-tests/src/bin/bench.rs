#![feature(plugin, const_fn)]
#![plugin(lia_plugin)]

#[macro_use]
extern crate lia;

use lia::runtime::*;

lia! {
    function fib(n) {
        if (n == 0) { return 1; }
        if (n == 1) { return 1; }
        return @fib(n-1) + @fib(n-2);
    }
}

fn main() {
    use std::time::Instant;
    let n = 30;
    let start = Instant::now();
    let result = call!(fib(n));
    cast!(v, result, i32);
    let duration = start.elapsed();
    let time = (duration.as_secs() as f32) + (duration.subsec_nanos() as f32) / 10e9;
    println!("Result: {} (took {}s)", v, time);
}
