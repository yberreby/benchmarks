#![feature(test)]
extern crate test;
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
        //assert!(list.iter().all(|x| x % 2 == 0));
    });
}


#[bench]
fn bench_even_iter(b: &mut Bencher) {
    b.iter(|| {
        let list = (0..LARGE_NUMBER).filter(|x| x % 2 == 0).collect::<Vec<_>>();
        
        list
    });
}
