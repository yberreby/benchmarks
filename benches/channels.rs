#![feature(test, alloc, heap_api)]

extern crate test;
extern crate rand;
extern crate crossbeam;
extern crate chan;

use std::sync::Arc;
use std::sync::mpsc;
use std::thread;

use rand::Rng;
use test::Bencher;

fn get_data() -> Vec<u8> {
    rand::thread_rng().gen_iter().take(1000).collect()
}

fn get_data_sum<I: IntoIterator<Item = u8>>(xs: I) -> u64 {
    xs.into_iter().fold(0, |sum, x| sum + (x as u64))
}

#[bench]
fn mpsc_chan_async_alt(b: &mut Bencher) {
    let data = Arc::new(get_data());
    let sum = get_data_sum(data.iter().cloned());
    b.bytes = data.len() as u64;

    let data = data.clone();
    let (s, r) = chan::async();

    crossbeam::scope(|scope| {
        scope.spawn(move || {
            b.iter(|| test::black_box(r.recv().unwrap()));
        });
    });

    thread::spawn(move || {
        for i in 0..1_000_000_000 {
            s.send(i);
        }
    });
}
