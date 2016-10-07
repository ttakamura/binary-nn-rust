use backend::bitpack::Bitpack32;
use backend::bitpack::Bitpack;
use std::slice::Iter;
use std::slice::IterMut;

pub trait BitOperation2 {
  fn process(&self, left: Bitpack32, right: Bitpack32) -> Bitpack32;
}

pub trait BitIterator {
  fn bitlen(&self) -> u32;
}

// ----------------------------------------------
pub struct BitOpXnor;

impl BitOperation2 for BitOpXnor {
  fn process(&self, left: Bitpack32, right: Bitpack32) -> Bitpack32 {
    left.xnor(&right)
  }
}

// ----------------------------------------------
pub struct BitIterZip<I, O>
  where I: BitIterator + Iterator,
        O: BitOperation2
{
  op: O,
  left: I,
  right: I,
}

impl<I, O> Iterator for BitIterZip<I, O>
  where I: BitIterator + Iterator<Item = Bitpack32>,
        O: BitOperation2
{
  type Item = Bitpack32;

  fn next(&mut self) -> Option<Self::Item> {
    match (self.left.next(), self.right.next()) {
      (Some(a), Some(b)) => Some(self.op.process(a, b)),
      _ => None,
    }
  }
}

// ----------------------------------------------
pub struct BitIter<'a> {
  raw: Iter<'a, Bitpack32>,
  bitlen: u32,
}

impl<'a> Iterator for BitIter<'a> {
  type Item = Bitpack32;

  fn next(&mut self) -> Option<Self::Item> {
    match self.raw.next() {
      Some(x) => Some(x.clone()),
      _ => None,
    }
  }
}

impl<'a> BitIterator for BitIter<'a> {
  fn bitlen(&self) -> u32 {
    self.bitlen
  }
}

impl<'a> BitIter<'a> {
  pub fn new(iter: Iter<Bitpack32>, bitlen: u32) -> BitIter {
    BitIter {
      raw: iter,
      bitlen: bitlen,
    }
  }
}

// ----------------------------------------------
pub struct BitIterMut<'a> {
  raw: IterMut<'a, Bitpack32>,
  bitlen: u32,
}

impl<'a> Iterator for BitIterMut<'a> {
  type Item = &'a mut Bitpack32;

  fn next(&mut self) -> Option<Self::Item> {
    self.raw.next()
  }
}

impl<'a> BitIterMut<'a> {
  pub fn new(iter: IterMut<Bitpack32>, bitlen: u32) -> BitIterMut {
    BitIterMut {
      raw: iter,
      bitlen: bitlen,
    }
  }

  pub fn union(&mut self, other: BitIter) {
    self.mut_process(other, |a, b| a.mut_union(b))
  }

  pub fn intersect(&mut self, other: BitIter) {
    self.mut_process(other, |a, b| a.mut_intersect(b))
  }

  pub fn xor(&mut self, other: BitIter) {
    self.mut_process(other, |a, b| a.mut_xor(b))
  }

  pub fn xnor(&mut self, other: BitIter) {
    self.mut_process(other, |a, b| a.mut_xnor(b))
  }

  pub fn mut_process<F>(&mut self, mut other: BitIter, mut op: F)
    where F: FnMut(&mut Bitpack32, &Bitpack32)
  {
    for x in self {
      let y = other.next();
      op(x, &y.unwrap());
    }
  }
}
