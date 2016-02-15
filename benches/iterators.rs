#![feature(test)]
extern crate test;
extern crate libc;

use test::Bencher;

const LARGE_NUMBER: i32 = 1_000_000;



#[bench]
fn bench_even_imperative(b: &mut Bencher) {
    b.iter(|| {
        let mut list = Vec::with_capacity(LARGE_NUMBER as usize / 2 + 1);

        let mut i = 0;

        while i <= LARGE_NUMBER {
            list.push(i * 2);
            i += 1;
        }

        // if the code below is uncommented the next benchmark almost
        // runs twice as fast...?
        if test::black_box(false) {
            unsafe {
                libc::puts(1 as *const i8);
            }
        }
    });
}


#[bench]
fn bench_even_iter(b: &mut Bencher) {
    b.iter(|| {
        let list = (0..LARGE_NUMBER).filter(|x| x % 2 == 0).collect::<Vec<_>>();

        list
    });
}
