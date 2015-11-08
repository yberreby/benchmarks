#![feature(test)]
extern crate test;
use test::Bencher;

extern crate rand;

use std::cell::RefCell;
use rand::os::OsRng;
use rand::{Rand, Rng, SeedableRng};
use rand::isaac::IsaacRng;

const SEED: [u32; 4] = [4634, 124, 3197, 94];

thread_local!(static OS_RNG: RefCell<OsRng> = RefCell::new(OsRng::new().unwrap()));


fn os_random<T: Rand>() -> T {
  OS_RNG.with(|r| {
    r.borrow_mut().gen()
  })
}


#[bench]
fn bench_random(b: &mut Bencher) {
  b.iter(|| test::black_box(
    rand::random::<[u8; 32]>()
  ));
}

#[bench]
fn bench_tl_os_random(b: &mut Bencher) {
  let a: u8 = os_random();
  test::black_box(a);

  b.iter(|| test::black_box(
    os_random::<[u8; 32]>()
  ));
}

#[bench]
fn bench_osrng(b: &mut Bencher) {
  let mut os_rng = OsRng::new().unwrap();

  b.iter(|| test::black_box(
    os_rng.gen::<[u8; 32]>()
  ));
}

#[bench]
fn bench_osrng_new(b: &mut Bencher) {
  b.iter(|| test::black_box(
    OsRng::new().unwrap()
  ));
}

#[bench]
fn bench_isaacrng_new_unseeded(b: &mut Bencher) {
  b.iter(|| test::black_box(
    IsaacRng::new_unseeded()
  ));
}


#[bench]
fn bench_isaacrng_from_seed(b: &mut Bencher) {
  b.iter(|| test::black_box(
    IsaacRng::from_seed(&SEED)
  ));
}

#[bench]
fn bench_isaacrng(b: &mut Bencher) {
  let mut rng = IsaacRng::from_seed(&SEED);

  b.iter(|| test::black_box(
    rng.gen::<[u8; 32]>()
  ));
}
