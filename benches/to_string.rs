#![feature(test)]
extern crate test;
use test::Bencher;


#[bench]
fn bench_integer_to_string(b: &mut Bencher) {
    b.iter(|| 1232.to_string());
}


#[bench]
fn bench_float_to_string(b: &mut Bencher) {
    b.iter(|| 1232.5768787779797.to_string());
}
