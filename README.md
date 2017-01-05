# Lia: A High-Level Language for Rust

> 俩 (liǎ) - 1. two, 2. a pair

Lia is a dynamically typed and garbage collected programming language that seamlessly interoperates with Rust by using Rust as a compile target. This enables Lia users to drop down into efficient Rust code when necessary, but work with a high-level Javascript-esque language for the majority of their application. For example, binding to a matrix library (à la numpy) is simple:

```rust
// lia! declares a set of Lia functions. It is a procedural macro that compiles Lia into Rust.
lia! {
    function multiply_matrices() {
        console.log("Multiplying matrices");
        var x = @Matrix::from_list([[4, 3], [2, 1]]); // The @ means a foreign (Rust) function
        var y = @Matrix::from_list([[1, 2], [3, 4]]);
        var z = @Matrix::multiply(x, y);
        return @Matrix::get(z, 0, 0);
    }
}

#[derive(Clone)]
struct Matrix {
    data: Vec<i32>,
    rows: i32,
    cols: i32,
}

// Putting #[lia_impl_glue] on an impl block will automatically generate functions that
// do the appropriate type-casting from Lia's dynamic types into Rust's static types.
#[lia_impl_glue]
impl Matrix {
    // some functions omitted...

    pub fn multiply(&self, other: &Matrix) -> Matrix {
        assert!(self.cols == other.rows);

        let mut new_mat = Matrix::new(self.rows, other.cols);
        for i in 0..self.rows {
            for j in 0..other.cols {
                let mut dot = 0;
                for k in 0..self.cols {
                    dot += self.get(i, k) * other.get(k, j);
                }
                new_mat.set(i, j, dot);
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

Look at `lia-tests/src/lib.rs` for example usage. Right now Lia is still in its early stages and requires nightly to build so it's not ready for prime time, but I encourage you to try it out and bring up any suggestions in an issue/PR or email me at [wcrichto@stanford.edu](mailto:wcrichto@stanford.edu).

To use Lia in your own code, first switch to Rust nightly with `multirust override nightly`. Then add to your `Cargo.toml`:

```toml
[dependencies]
lia = "0.1.0"
lia-plugin = "0.1.0"
```

Then in the file you want to use Lia:

```rust
#![feature(plugin, box_syntax)]
#![plugin(lia_plugin)]

#[macro_use]
extern crate lia;

use lia::runtime::*;

lia! { ... }
```

## Motivation

One of the biggest challenges facing programming languages today is interoperability. As PLs both proliferate in ever increasing numbers and specialize to application domains, many programmers need the ability to work effectively across languages. For example:

* Python has a plethora of bindings to C, CUDA, etc. for efficient operations on numeric or image data. This is implemented in libraries like [numpy](http://www.numpy.org/) and [scipy](http://www.scipy.org/).
* Javascript recently saw the rise of [React](http://reactjs.org/) which enabled seamless integration of HTML and Javascript in their new JSX format (this is very similar to the procedural HTML generation that popularized PHP many years earlier).
* A number of domain-specific languages, e.g. [Halide](http://halide-lang.org/), have popped up in recent years with varying levels of interoperability to existing general-purpose programming languages.

Many of the problems with interoperability between two languages can be solved by sharing a common type system. Even if two languages have a different runtime and different syntax, if they merge at the type-level then it's considerably easier for them to work together. For example, [Terra](http://terralang.org) addressed this problem by modifying Lua to have code generation facilities for a C-like language and then using LuaJIT's FFI as the type system glue between the two languages. However, this approach necessitates that the two languages (Lua and Terra) have separate runtimes and relies on a fragile layer of type glue between Lua's types and what is effectively C's types.

Lia takes a different approach: instead of separating the high level interpreted runtime and the low level compiled runtime, we compile Lia code into Rust code. That way it shares the same type system and runtime and enables seamless interoperability between the two languages. Lia raises the level of abstraction over Rust by eliminating lifetimes/memory management as well as static typing.

## System description

```rust
type LiaAny = Rc<RefCell<Rc<RefCell<Box<Any>>>>>;
```

All values in Lia (integers, closures, etc.) have the type `LiaAny`. We'll walk through the components of the above type to understand Lia's layers of abstraction.

### Eliminating memory management

The burden of managing memory/lifetimes is shifted from the programmer at compile time to the program at runtime via reference counting (type `Rc`). Ideally this would be a garbage collected pointer (like [this one](https://github.com/Manishearth/rust-gc)) that won't leak memory when cycles are induced, but that can come later. The `Rc` wraps a `RefCell` to allow the enclosed value to be mutated.

A simple implementation of a runtime-managed type could be just `Rc<RefCell<T>>`. However, we need a second layer of indirection because variables can be reassigned. Consider the following example:

```javascript
var x = 1;
x = 2;
```

With the naive implementation, this could compile as:

```rust
let mut x = alloc(1);
x = alloc(2);
```

However, we do not want to rely on the mutability of Rust variables (specifically the names, e.g. `x`, not the values) to enable the mutability of Lia. This arises when we consider the interaction between closures and mutability:

```javascript
var x = 1;
(function(){ x = 2; })();
x = 3;
```

Our above scheme would compile into:

```rust
let mut x = alloc(1);
(move || { x = alloc(2); })();
x = alloc(3);
```

And the compiler will complain on the third line that `x` is already moved into the closure. Instead, we have each Rust name correspond to a slot that contains a value. The above example actually compiles into roughly:

```rust
let x = alloc(alloc(1));
let x_copy = x.clone();
(move || { *x_copy.borrow_mut() = alloc(2); })();
*x.borrow_mut() = alloc(3);
```

The compiler creates the copies by finding variables closed by the Lia closures and creating copies. In sum: to have both runtime-managed memories and closures, each value is wrapped in two layers of `Rc<RefCell<...>`.

### Eliminating static typing

To make the language dynamically typed, each underlying value in the slots discussed above has type `Box<Any>`. The [`Any`](http://doc.rust-lang.org/stable/std/any/index.html) type represents a value of any possible type, and can be checked/casted at runtime. The `Box<...>` ensures that the type is a fixed size (similar to the OCaml runtime, where each value is the size of a pointer).

Casting in to the generic type is done with `lia::runtime::alloc` that does all the proper allocations. Casting out uses the `cast!` macro. Cast semantics depend on whether the type being casted into is owned or borrowed. The `Any` type from the standard library provides downcast methods that return a reference to the enclosed value when casted correctly. So when casting a `LiaAny` into a reference type, the `cast!` macro just uses the given reference. However, if you want to cast into an owned type, then the referenced value gets cloned. This is because it would be unsafe to take ownership from the Lia runtime, as Lia makes no guarantees on the number of references to a value at any point in time.

## TODO list
- [ ] Add JIT for dynamic execution
- [ ] Make errors clear on both the Lia level (Spans) and Rust level (code maps?)
- [ ] Import remaining language constructs
  - [ ] Loops
  - [ ] Rest of the binops
  - [ ] Modules
