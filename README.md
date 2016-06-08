# Lia: A High-Level Language For Rust

Lia is a programming language that enables expressive programming and rapid prototyping by eliminating memory management/lifetimes and static typing. Lia compiles down into Rust code, so it seamlessly interoperates with Rust libraries. This enables Lia users to drop down into efficient Rust code when necessary, but work with a high-level Javascript-esque language for the majority of their program. For example, binding to a matrix library (like numpy) is simple:

```
#![feature(plugin, box_syntax)]
#![plugin(lia_plugin)]

#[macro_use]
extern crate lia;

use lia::runtime::*;

// lia! declares a set of Lia functions. It is a procedural macro that compiles Lia into Rust.
lia! {
    function multiply_matrices() {
        var x = @Matrix::from_list([[4, 3], [2, 1]]); // The @ means a foreign (Rust) function
        var y = @Matrix::new([[1, 2], [3, 4]]);
        var z = @Matrix::multiply(x, y);
        return @Matrix::get(z, 0, 0);
    }
}

struct Matrix {
    data: Vec<i32>,
    rows: i32,
    cols: i32,
}

// Putting #[lia_impl_glue] on an impl block will automatically generate functions that
// do the appropriate type-casting from Lia's dynamic types into Rust's static types.
#[lia_impl_glue]
impl Matrix {
    pub fn new(rows: i32, cols: i32) -> Matrix {
        let mut data = Vec::with_capacity((rows * cols) as usize);
        for _ in 0..(rows * cols) {
            data.push(0);
        }

        Matrix {
            rows: rows,
            cols: cols,
            data: data,
        }
    }

    pub fn get(&self, row: i32, col: i32) -> i32 {
        self.data[(row * self.cols + col) as usize]
    }

    pub fn set(&mut self, row: i32, col: i32, val: i32) {
        self.data[(row * self.cols + col) as usize] = val;
    }

    pub fn multiply(&self, other: &Matrix) -> Matrix {
        assert!(self.cols == other.rows);

        let mut new_mat = Matrix::new(self.rows, other.cols);
        for i in 0..self.rows {
            for j in 0..other.cols {
                let mut dot = 0;
                for k in 0..self.cols {
                    dot += self.data[(i * self.cols + k) as usize] *
                        other.data[(k * other.cols + j) as usize];
                }
                new_mat.data[(i * new_mat.cols + j) as usize] = dot;
            }
        }

        return new_mat;
    }
}

fn main() {
    // Lia includes macros that simplify handling Lia functions and values in Rust.
    let result: LiaAny = call!(multiply_matrices());
    cast!(let num: i32 = result);
    assert!(num == 13);
}
```

## Using Lia

Look at `lia-tests/src` for example usage. Right now Lia is still in its early stages and requires nightly to build so it's not ready for prime time, but I encourage you to try it out and bring up any suggestions in an issue/PR or email me at [wcrichto@stanford.edu](mailto:wcrichto@stanford.edu).

## Motivation

One of the biggest challenges facing programming languages today is interoperability. As PLs both proliferate in ever increasing numbers and specialize to application domains, many programmers need the ability to work effectively across languages. For example:

* Python has a plethora of bindings to C, CUDA, etc. for efficient operations on numeric or image data. This is implemented in libraries like numpy and scipy.
* Javascript recently saw the rise of React which enabled seamless integration of HTML and Javascript in their new JSX format (this is very similar to the procedural HTML generation that popularized PHP many years earlier).
* A number of domain-specific languages, e.g. [Halide](http://halide-lang.org/), have popped up in recent years with varying levels of interoperability to existing general-purpose programming languages.

Many of the problems with interoperability between two languages can be solved by sharing a common type system. Even if two languages have a different runtime and different syntax, if they merge at the type-level then it's considerably easier for them to work together. For example, [Terra](http://terralang.org) addressed this problem by modifying Lua to have code generation facilities for a C-like language and then using LuaJIT's FFI as the type system glue between the two languages. However, this approach necessitates that the two languages (Lua and Terra) have separate runtimes and relies on a fragile layer of type glue between Lua's types and what is effectively C's types.

Lia takes a different approach: instead of separating the high level interpreted runtime and the low level compiled runtime, we compile Lia code into Rust code. That it way it shares the same type system and runtime and enables seamless interoperability between the two languages.