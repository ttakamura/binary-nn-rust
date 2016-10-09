use backend::bitpack::Bitpack32;
use backend::bitpack::Bitpack;
use std::slice::Iter;

pub trait BitOperation2
  where Self: Clone
{
  fn process(&self, left: Bitpack32, right: Bitpack32) -> Bitpack32;
}

pub trait BitIterator
  where Self: Iterator<Item = Bitpack32> + Sized + Clone
{
  type Index: PartialEq + Clone;

  fn union(&self, other: &Self) -> BitIterZip<Self, BitOpUnion> {
    let op = BitOpUnion {};
    return BitIterZip::new(op, self.clone(), other.clone());
  }

  fn xnor(&self, other: &Self) -> BitIterZip<Self, BitOpXnor> {
    let op = BitOpXnor {};
    return BitIterZip::new(op, self.clone(), other.clone());
  }

  fn nbits(&self) -> Self::Index;
}

// ----------------------------------------------
#[derive(Clone)]
pub struct BitOpXnor;

impl BitOperation2 for BitOpXnor {
  fn process(&self, left: Bitpack32, right: Bitpack32) -> Bitpack32 {
    left.xnor(&right)
  }
}

#[derive(Clone)]
pub struct BitOpUnion;

impl BitOperation2 for BitOpUnion {
  fn process(&self, left: Bitpack32, right: Bitpack32) -> Bitpack32 {
    left.union(&right)
  }
}

// ----------------------------------------------
pub struct BitIterZip<I, O>
  where I: BitIterator,
        O: BitOperation2
{
  op: O,
  left: I,
  right: I,
}

impl<I, O> Clone for BitIterZip<I, O>
  where I: BitIterator,
        O: BitOperation2
{
  fn clone(&self) -> Self {
    BitIterZip::new(self.op.clone(), self.left.clone(), self.right.clone())
  }
}

impl<I, O> Iterator for BitIterZip<I, O>
  where I: BitIterator,
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

impl<I, O> BitIterator for BitIterZip<I, O>
  where I: BitIterator,
        O: BitOperation2
{
  type Index = I::Index;

  fn nbits(&self) -> I::Index {
    self.left.nbits()
  }
}

impl<I, O> BitIterZip<I, O>
  where I: BitIterator,
        O: BitOperation2
{
  pub fn new(op: O, left: I, right: I) -> Self {
    if left.nbits() != right.nbits() {
      panic!("iter.nbits should be the same length");
    }
    BitIterZip {
      op: op,
      left: left,
      right: right,
    }
  }
}

// ----------------------------------------------
pub struct BitIter<'a> {
  raw: Iter<'a, Bitpack32>,
  nbits: u32,
}

impl<'a> Clone for BitIter<'a> {
  fn clone(&self) -> Self {
    BitIter::new(self.raw.clone(), self.nbits.clone())
  }
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
  type Index = u32;

  fn nbits(&self) -> Self::Index {
    self.nbits
  }
}

impl<'a> BitIter<'a> {
  pub fn new(iter: Iter<Bitpack32>, nbits: u32) -> BitIter {
    BitIter {
      raw: iter,
      nbits: nbits,
    }
  }
}
