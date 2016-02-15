#![feature(test)]
#![feature(str_char)]
extern crate test;
extern crate libc;

use test::Bencher;


#[bench]
fn increment(b: &mut Bencher) {
    b.iter(|| {
        let mut result = 0;

        for i in 0..1_000_000 {
            result += test::black_box(1);
        }

        result
    });
}
