#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
fn plus_one(a: i32) -> i32 {
    a + 1
}
fn show_func_01() {
    {
        ::std::io::_print(
            match match () {
                () => [],
            } {
                ref args => unsafe {
                    ::core::fmt::Arguments::new_v1(&["show_fun 01: std\n"], args)
                },
            },
        );
    };
}

fn get_value() -> u32 {
    let a = 10u32;
    let b = 23u32;
    a + b
}

fn test() {
    let a = 10;
    if !(a > 10) {
        {
            ::std::rt::begin_panic("a <= 10")
        }
    };
}
fn main() {
    test();
}
