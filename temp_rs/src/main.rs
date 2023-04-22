
#![feature(type_ascription)]
#![feature(test)]
#![feature(iter_collect_into)]

mod a;

extern crate test;

use std::mem::MaybeUninit;
use std::time::Instant;
use glam::*;
use crate::a::run;


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

    // let now = Instant::now();
    // run();
    // println!("Elapsed: {:.8?}", now.elapsed());

    [
        Mat4::IDENTITY,
        // Mat4::from_rotation_x(x),
        // Mat4::from_rotation_y(y),
        // Mat4::from_scale(vec3(x, x, 1.0)),
        // Mat4::from_translation(vec3(x, y, 1.0)),
        // Mat4::look_at_rh(vec3(2.0, 2.0, 2.0), vec3(0.0, 0.0, 0.0), vec3(0.0, 0.0, 1.0)),
        // Mat4::perspective_rh_gl(0.45 * 1.745329, W as f32 / H as f32, 0.1, 10.0)
    ];


    let a = Mat4::from_rotation_y(1.);
    println!("{:2.4}, {:2.4}, {:2.4}, {:2.4}", a.x_axis[0], a.x_axis[1], a.x_axis[2], a.x_axis[3]);
    println!("{:2.4}, {:2.4}, {:2.4}, {:2.4}", a.y_axis[0], a.y_axis[1], a.y_axis[2], a.y_axis[3]);
    println!("{:2.4}, {:2.4}, {:2.4}, {:2.4}", a.z_axis[0], a.z_axis[1], a.z_axis[2], a.z_axis[3]);
    println!("{:2.4}, {:2.4}, {:2.4}, {:2.4}", a.w_axis[0], a.w_axis[1], a.w_axis[2], a.w_axis[3]);


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
    // let mut c = MaybeUninit::<A>::uninit();
    // let x;
    // unsafe {
    //     c.write(A {a1: 23, a0: B {b0: "233".to_string()} });
    //     (*c.as_mut_ptr()).a0.b0 = "322".to_string();
    //     println!("{}", (*c.as_ptr()).a0.b0);
    //     x = c.assume_init();
    // }
    //
    // let a0 = A {a0: B {b0: "2".to_string()}, a1: 6 };
    // let mut a0 = A {a0: B {b0: "3".to_string()}, a1: 6 };
    // let mut a1 = A {a0: b, a1: 6 };
    // a1.a0 = B {b0: "4".to_string()};
    // a0.a0 = B {b0: "5".to_string()};

    // let a = [Some((233, "abc")), None];
    // let a = [Ok((233, "abc")), Err(233)];
    // let b = a[0].map(|x| { println!("1"); x.0});
    // let c = a[1].map(|x| { println!("2"); x.0});
    // let d = a[0].map_or(0, |x| { println!("3"); x.0});
    // let e = a[1].map_or(0, |x| { println!("4"); x.0});
    // let f = a[0].map_err(|x| { println!("5"); 244});
    // let g = a[1].map_err(|x| { println!("6"); 244});
    // println!("- {b:?}");
    // println!("- {c:?}");
    // println!(" -{d:?}");
    // println!(" -{e:?}");
    // println!("- {f:?}");
    // println!("- {g:?}");


}