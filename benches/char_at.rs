#![feature(test)]
#![feature(str_char)]
extern crate test;
extern crate libc;

use test::Bencher;


const LARGE_STRING: &'static str = "ZcIqpuJAusRDhIHIZAu4atu5ZQG7aecMeym8Nt96s1tag8Qe1o9vwLeUQiIr
\
                                    2v74l5SVLipKPlsopU4eFmm3nIKscIuldykX6O1eA9Qo4H0mKJHOK9doM0ALQg\
                                    esDJ3AWjcn9ae6fpyFqhKX7kBTsdQsLE3e
\
                                    HuQ8zOsW4SgeqeJunZW3BYnJR7aChXNoTmY8QeoFi0uJ";


fn char_at(s: &str, i: usize) -> char {
    s[i..].chars().next().unwrap()
}

#[bench]
fn bench_char_at_iter(b: &mut Bencher) {
    b.iter(|| test::black_box(char_at(LARGE_STRING, 50)));
}
