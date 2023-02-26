
#![feature(type_ascription)]
#![feature(test)]
extern crate test;

use std::mem::MaybeUninit;
use ash::extensions::{ext, khr};
use ash::{vk, *};


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
    println!("---");

    let entry = unsafe { Entry::load().expect("miao") };
    let app_info = vk::ApplicationInfo {
        api_version: vk::make_api_version(0, 1, 0, 0),
        ..Default::default()
    };
    let create_info = vk::InstanceCreateInfo {
        p_application_info: &app_info,
        ..Default::default()
    };
    // vk::InstanceCreateInfo
    let instance = unsafe { entry.create_instance(&create_info, None).expect("miao") };

    // let a = [Some((233, "abc")), None];
    // let b = a[0].map(|x| x.0);
    // let c = a[1].map(|x| x.0);
    // println!("{b:?}");
    // println!("{c:?}");
}