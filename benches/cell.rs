#![feature(test)]
extern crate test;
use test::Bencher;

use std::cell::{RefCell, Cell};

#[bench]
fn cell_increment_1000(b: &mut Bencher) {
  let cell = Cell::new(0);
  b.iter(|| {
    let n = 1000;
    for _ in 0..n {
      cell.set(cell.get() + test::black_box(1));
    }
    cell.get()
  });
}

#[bench]
fn refcell_increment_1000(b: &mut Bencher) {
  let refcell = RefCell::new(0);
  b.iter(|| {
    let n = 1000;
    for _ in 0..n {
      let old = *refcell.borrow();
      *refcell.borrow_mut() = old + test::black_box(1);
    }
    *refcell.borrow()
  });
}



#[bench]
fn regular_increment_1000(b: &mut Bencher) {
  let mut val = 0;

  b.iter(|| {
    let n = test::black_box(1000);
    for _ in 0..n {
      val += test::black_box(1);
    }
    val
  });
}
