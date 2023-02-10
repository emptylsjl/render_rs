
#![feature(type_ascription)]



fn main() {
    let this_file = include_str!("main.rs");
    println!("{this_file}");

    let current_col = column!();
    println!("defined on column: {current_col}");

    let a = ("foobar", column!());
    let c = ("f̅o̅o̅b̅a̅r̅", column!());

    println!("{a:?}");
    println!("{c:?}");


}