#![feature(plugin)]
#![plugin(lia_lang)]

lia!(function foo() {
    var x = 1;
});

#[test]
fn macro_test() {
}
