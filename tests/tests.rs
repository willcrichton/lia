#![feature(plugin)]
#![plugin(lia_lang)]


#[test]
fn macro_test() {
    println!("{}", lia!(1 + 2));

    // match grammar::parse_expr("1 + 2") {
    //     Ok(val) => println!("{}", cast!(val, i32)),
    //     Err(_) => println!(":(")
    // }
}
