// #![feature(prelude_import)]
// #[prelude_import]
// use std::prelude::rust_2018::*;
// extern  crate std;
// use std::rt::begin_panic;
// #![cfg_attr(not(feature = "std"), no_std)]
// #![cfg_attr(feature="std", feature("foo"))]
#![allow(dead_code)]

fn plus_one(a: i32)->i32{
    a + 1
}

// #[cfg(feature="std")]
fn show_func_01(){
    #[cfg(feature="std")]
    println!("show_fun 01: std");

    // #[cfg(not(feature="std"))]
    // #[cfg_attr(not(feature="std"), feature("foo"))]
    // println!("foo func 01_1");

    #[cfg(feature="foo")]
    println!("show_fun 01: foo");
}

fn get_value()->u32{
    let a = 10u32;
    let b = 23u32;
    a+b
}

fn test(){
    // let a = 10;
    // let b = 20;
    // let n = plus_one(10);
    // println!("{}", n);
    // show_func_01();
    // let _ = get_value();

    let a = 10;
    assert!(a>10, "a <= 10");

    // if a>10{
    //     begin_panic("a<10");
    // }

    // my_cmp!(a, b);
    // show_func_02();
    // show_msg!();
}

// use std::{future::Future, pin::Pin, task::Context};

// #[derive(Default)]
// struct RandFuture;

// impl Future for RandFuture{
//     type Output = u16;

//     fn poll(self: Pin<&mut Self>, _: &mut Context)-> Poll<Self::Output>{
//         Poll::ready(rand::random())
//     }
// }

#[derive(Debug)]
struct TestStruct{
    a: String,
    b: *const String,
}

impl TestStruct{
    fn new(txt: &str) -> Self{
        Self {
            a: String::from(txt),
            b: std::ptr::null(),
        }
    }

    fn init(&mut self){
        let self_ref: *const String = &self.a;
        self.b = self_ref;
    }

    fn a(&self)->&str{
        return &self.a;
    }

    fn b(&self)->&String{
        return unsafe{ &*(self.b)};
    }
}

fn test01(){
    let mut t1 = TestStruct::new("111111");
    t1.init();
    let mut t2 = TestStruct::new("AAAAAA");
    t2.init();

    println!("{}, {}", t1.a(), t1.b());
    println!("{}, {}", t2.a(), t2.b());

    std::mem::swap(&mut t1, &mut t2);
    // std::mem::swap(&mut t1.a, &mut t2.a);
    println!("{}, {}", t1.a(), t1.b());
    println!("{}, {}", t2.a(), t2.b());

    // t1.a = String::from("TTTTTTT");
    // println!("{}, {}", t2.a(), t2.b());
}

fn test02(){
    let mut t1 = TestStruct::new("111111111");
    t1.init();

    println!("{}, {}", t1.a(), t1.b());
    t1.a = String::from("MMMMMMMMM");
    println!("{}, {}", t1.a(), t1.b());
}

fn main() {
    test01();
    // test02();
    // println!("Hello, world!");
}
