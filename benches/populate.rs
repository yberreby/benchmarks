#![feature(test)]
extern crate test;
use test::Bencher;

const COUNT: usize = 150;

#[bench]
fn populate_for(b: &mut Bencher) {
  b.iter(|| {
    let mut output = Vec::with_capacity(COUNT);

    for _ in 0..COUNT {
      output.push(String::new());
    }

    output
  });
}

#[bench]
fn populate_map(b: &mut Bencher) {
  b.iter(|| {
    (0..COUNT).map(|_| String::new()).collect::<Vec<_>>()
  });
}
