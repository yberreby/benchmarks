#![feature(test)]
extern crate test;
use test::Bencher;


#[bench]
fn bench_thread(b: &mut Bencher) {
    b.iter(|| std::thread::spawn(move || {}));
}
