#![no_std]
#![feature(test)]

extern crate alloc;
extern crate test;
use alloc::{format, string::String};
use test::{black_box, Bencher};
use unchecked_std::prelude::*;

fn hello_format(name: &str) -> String {
    format!("Hello, {name}!")
}

fn hello_checked(name: &str) -> String {
    let mut s = String::with_capacity("Hello, !".len() + name.len());
    s.push_str("Hello, ");
    s.push_str(name);
    s.push('!');
    s
}

fn hello_unchecked(name: &str) -> String {
    let mut s = String::with_capacity("Hello, !".len() + name.len());
    unsafe {
        s.push_str_unchecked("Hello, ");
        s.push_str_unchecked(name);
        s.push_unchecked('!');
    }
    s
}

#[bench]
fn bench_hello_format(bencher: &mut Bencher) {
    bencher.iter(|| hello_format(black_box("Rust")));
}

#[bench]
fn bench_hello_checked(bencher: &mut Bencher) {
    bencher.iter(|| hello_checked(black_box("Rust")));
}

#[bench]
fn bench_hello_unchecked(bencher: &mut Bencher) {
    bencher.iter(|| hello_unchecked(black_box("Rust")));
}
