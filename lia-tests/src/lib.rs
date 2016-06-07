#![feature(plugin, box_syntax)]
#![plugin(lia_plugin)]

#[macro_use]
extern crate lia;

use lia::runtime::*;

struct Matrix {
    data: Vec<i32>,
    rows: i32,
    cols: i32,
}

#[lia_impl_glue]
impl Matrix {
    pub fn new() -> Matrix {
        Matrix {
            rows: 0,
            cols: 0,
            data: Vec::new(),
        }
    }

    pub fn multiply(&self, other: &Matrix) -> Matrix {
        if self.cols != other.rows {
            // TODO: what if this returned a Result instead?
            panic!("dimension mismatch");
        }

        // implement efficient mat mul here

        return Matrix::new();
    }
}

lia! {
    function foo() {
        var x = @Matrix::new();
        var y = @Matrix::new();
        var z = @Matrix::multiply(x, y);
        return 0;
    }
}

#[test]
fn macro_test() {
    let result: LiaAny = call!(foo());
    cast!(let num: i32 = result);
    println!("result {}", num);
}
