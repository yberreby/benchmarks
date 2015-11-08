#![feature(test)]
extern crate test;
use test::Bencher;


const EXAMPLE: &'static str = "apdqzhouzqgzqdqz4dqzfzqf4qzdq6787n8d";

#[bench]
fn bench_as_bytes(b: &mut Bencher) {
  b.iter(|| test::black_box(EXAMPLE.as_bytes()));
}

#[bench]
fn bench_as_bytes_cached(b: &mut Bencher) {
  let example_as_bytes = EXAMPLE.as_bytes();
  b.iter(|| test::black_box(example_as_bytes));
}
