#![feature(test)]
extern crate test;
extern crate libc;

use test::{Bencher, black_box};
use std::f64::consts::PI;

// #[bench]
// fn bench_plain(b: &mut Bencher) {
//    b.iter(|| plain(LARGE));
// }

fn large_vec() -> Vec<f64> {
    const SIZE: usize = 100_000;
    let mut v = Vec::with_capacity(SIZE);

    for i in 0..SIZE {
        // 'Random enough' data, yet deterministic.
        v.push(expensive((i * 1000) as f64 / 68485.16377743));
    }

    v
}

#[bench]
fn bench_expensive(b: &mut Bencher) {
    b.iter(|| expensive(black_box(1.36468745454741984138474373461572756241945276)));
}


#[bench]
fn bench_plain(b: &mut Bencher) {
    let mut v = large_vec();
    b.iter(|| plain(&mut v, expensive));
}


#[bench]
fn bench_unwrapping(b: &mut Bencher) {
    let mut v: Vec<Option<f64>> = large_vec().into_iter().map(|x| Some(x)).collect();
    b.iter(|| unwrapping(&mut v, expensive));
}


#[bench]
fn bench_plain_cheap(b: &mut Bencher) {
    let mut v = large_vec();
    b.iter(|| plain(&mut v, cheap));
}


#[bench]
fn bench_unwrapping_cheap(b: &mut Bencher) {
    let mut v: Vec<Option<f64>> = large_vec().into_iter().map(|x| Some(x)).collect();
    b.iter(|| unwrapping(&mut v, cheap));
}



// Represents an expensive floating-point operation.
#[inline(never)]
fn expensive(x: f64) -> f64 {
    (x + 1.98 / (x) - x.cos() + (x + PI / 2.732).tan()).exp_m1().sqrt()
}

#[inline(never)]
fn cheap(x: f64) -> f64 {
    x * 5.6
}

#[inline(never)]
fn plain(data: &mut [f64], op: fn(f64) -> f64) {
    for elem in data {
        *elem = op(*elem);
    }
}

#[inline(never)]
fn unwrapping(data: &mut [Option<f64>], op: fn(f64) -> f64) {
    for elem in data {
        *elem = Some(op(elem.unwrap()));
    }
}
