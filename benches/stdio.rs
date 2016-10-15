//! WARNING: running this benchmark will output a huge amount of data to stdout.

#![feature(test)]
#![feature(str_char)]
extern crate test;
extern crate libc;

use test::Bencher;
use std::io;
use std::io::Write;

const TEST_STR: &'static str = "hello";

#[bench]
fn print_macro(b: &mut Bencher) {
    b.iter(|| {
        print!("{}", TEST_STR);
    });
}

#[bench]
fn print_macro_locked_stdout(b: &mut Bencher) {
    let mut stdout = std::io::stdout();
    let stdout = stdout.lock();
    b.iter(|| {
        print!("{}", TEST_STR);
    });
    print!("print_macro_locked_stdout");
}

#[bench]
fn direct_unlocked_stdout(b: &mut Bencher) {
    let mut stdout = io::stdout();
    b.iter(|| {
        stdout.write(TEST_STR.as_bytes());
    });

    print!("direct_unlocked_stdout");
}

#[bench]
fn direct_locked_stdout(b: &mut Bencher) {
    let mut stdout = io::stdout();
    let mut stdout = stdout.lock();
    b.iter(|| {
        stdout.write(TEST_STR.as_bytes());
    });

    print!("direct_locked_stdout");
}
