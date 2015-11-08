#![feature(test, alloc, heap_api)]

extern crate test;
use test::Bencher;

extern crate alloc;

#[bench]
fn bench_alloc(b: &mut Bencher) {
    b.iter(|| unsafe {
      alloc::heap::allocate(4, 64)
    });
}
