#![feature(test, core, core_panic)]
extern crate test;
extern crate core;
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

        // if the line below is uncommented the next benchmark almost
        // runs twice as fast...?
        //if !(list.iter().count() > 0) {
        //    panic!(concat!("assertion failed: ", "list.iter().count() > 0" ))
        //}
        if list.iter().count() == 0 {
            static _MSG_FILE_LINE: (&'static str, &'static str, u32) = ("explicit panic", file!(), line!());
            //core::panicking::panic(&_MSG_FILE_LINE)
            test::black_box(&_MSG_FILE_LINE);
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
