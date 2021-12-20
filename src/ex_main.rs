#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
fn show_func_01() {
    {
        ::std::io::_print(
            match match () {
                () => [],
            } {
                ref args => unsafe { ::core::fmt::Arguments::new_v1(&["std func 01\n"], args) },
            },
        );
    };
    {
        ::std::io::_print(
            match match () {
                () => [],
            } {
                ref args => unsafe { ::core::fmt::Arguments::new_v1(&["foo func 01_1\n"], args) },
            },
        );
    };
}
fn test() {
    let a = 10;
    let b = 20;
    if a == b {
        {
            ::std::io::_print(
                match match (&a, &b) {
                    (arg0, arg1) => [
                        ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                        ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                    ],
                } {
                    ref args => unsafe {
                        ::core::fmt::Arguments::new_v1(&["eq: ", "=", "\n"], args)
                    },
                },
            );
        };
    } else {
        {
            ::std::io::_print(
                match match (&a, &b) {
                    (arg0, arg1) => [
                        ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                        ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                    ],
                } {
                    ref args => unsafe {
                        ::core::fmt::Arguments::new_v1(&["ne: ", "!=", "\n"], args)
                    },
                },
            );
        };
    };
    show_func_01();
    let ma = 10;
    let mb = 20;
    let ret = ma + mb;
    {
        ::std::io::_print(
            match match (&ret,) {
                (arg0,) => [::core::fmt::ArgumentV1::new(
                    arg0,
                    ::core::fmt::Display::fmt,
                )],
            } {
                ref args => unsafe { ::core::fmt::Arguments::new_v1(&["xxxxx: ", "\n"], args) },
            },
        );
    };
    {
        ::std::io::_print(
            match match () {
                () => [],
            } {
                ref args => unsafe { ::core::fmt::Arguments::new_v1(&["????\n"], args) },
            },
        );
    };
}
fn main() {
    test();
}
