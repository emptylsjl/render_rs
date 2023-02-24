
#![feature(type_ascription)]
#![feature(test)]
extern crate test;


#[derive(Debug)]
struct A {
    a0: B,
    a1: u8
}

impl Drop for A {
    fn drop(&mut self) {
        println!("{:?}", self);
    }
}

#[derive(Debug)]
struct B {
    b0: String
}

impl Drop for B {
    fn drop(&mut self) {
        println!("{:?}", self);
    }
}

fn main() {
    // let this_file = include_str!("main.rs");
    // println!("{this_file}");
    //
    // let current_col = column!();
    // println!("defined on column: {current_col}");
    //
    // let a = ("foobar", column!());
    // let c = ("f̅o̅o̅b̅a̅r̅", column!());
    //
    // println!("{a:?}");
    // println!("{c:?}");

    // let b = B {b0: "1".to_string()};
    //
    // let a0 = A {a0: B {b0: "2".to_string()}, a1: 6 };
    // let mut a0 = A {a0: B {b0: "3".to_string()}, a1: 6 };
    // let mut a1 = A {a0: b, a1: 6 };
    // a1.a0 = B {b0: "4".to_string()};
    // a0.a0 = B {b0: "5".to_string()};
    // println!("---");

}