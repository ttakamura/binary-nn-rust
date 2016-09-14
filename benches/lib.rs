#![feature(test)]
extern crate test;
extern crate binary_nn;
use binary_nn::sandbox::*;
use test::Bencher;

fn call_add() {
  for _ in 0..10000 {
    add(100, 200);
  }
}

#[bench]
fn bench_add(b: &mut Bencher) {
  b.iter(|| call_add());
}
