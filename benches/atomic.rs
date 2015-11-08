#![feature(test)]
extern crate test;
use test::Bencher;

use std::sync::Mutex;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

#[bench]
fn bench_atomicusize(b: &mut Bencher) {
  let au = AtomicUsize::new(10);

  b.iter(|| {
    au.fetch_add(1, Ordering::Relaxed)
  });

  assert!(au.load(Ordering::Relaxed) > 10);
}

#[bench]
fn bench_mutex(b: &mut Bencher) {
  let au = Mutex::new(10);

  b.iter(|| {
    *au.lock().unwrap() += 1;
  });

  assert!(*au.lock().unwrap() > 10);
}
