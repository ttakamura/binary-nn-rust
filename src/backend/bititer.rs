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

  fn union<I>(&self, other: &I) -> BitIterZip<Self, I, BitOpUnion, Self::Index>
    where I: BitIterator<Item = Bitpack32, Index = Self::Index>
  {
    let op = BitOpUnion {};
    return BitIterZip::new(op, self.clone(), other.clone());
  }

  fn xnor<I>(&self, other: &I) -> BitIterZip<Self, I, BitOpXnor, Self::Index>
    where I: BitIterator<Item = Bitpack32, Index = Self::Index>
  {
    let op = BitOpXnor {};
    return BitIterZip::new(op, self.clone(), other.clone());
  }

  fn count_ones(&self) -> u32 {
    self.clone().fold(0, |acc, x| acc + x.count_ones())
  }

  fn shape(&self) -> Self::Index;
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
pub struct BitIterZip<IL, IR, O, I>
  where IL: BitIterator<Index = I>,
        IR: BitIterator<Index = I>,
        O: BitOperation2,
        I: PartialEq + Clone
{
  op: O,
  left: IL,
  right: IR,
}

impl<IL, IR, O, I> Clone for BitIterZip<IL, IR, O, I>
  where IL: BitIterator<Index = I>,
        IR: BitIterator<Index = I>,
        O: BitOperation2,
        I: PartialEq + Clone
{
  fn clone(&self) -> Self {
    BitIterZip::new(self.op.clone(), self.left.clone(), self.right.clone())
  }
}

impl<IL, IR, O, I> Iterator for BitIterZip<IL, IR, O, I>
  where IL: BitIterator<Index = I>,
        IR: BitIterator<Index = I>,
        O: BitOperation2,
        I: PartialEq + Clone
{
  type Item = Bitpack32;

  fn next(&mut self) -> Option<Self::Item> {
    match (self.left.next(), self.right.next()) {
      (Some(a), Some(b)) => Some(self.op.process(a, b)),
      _ => None,
    }
  }
}

impl<IL, IR, O, I> BitIterator for BitIterZip<IL, IR, O, I>
  where IL: BitIterator<Index = I>,
        IR: BitIterator<Index = I>,
        O: BitOperation2,
        I: PartialEq + Clone
{
  type Index = IL::Index;

  fn shape(&self) -> IL::Index {
    self.left.shape()
  }
}

impl<IL, IR, O, I> BitIterZip<IL, IR, O, I>
  where IL: BitIterator<Index = I>,
        IR: BitIterator<Index = I>,
        O: BitOperation2,
        I: PartialEq + Clone
{
  pub fn new(op: O, left: IL, right: IR) -> Self {
    if left.shape() != right.shape() {
      panic!("iter.shape should be the same length");
    }
    BitIterZip {
      op: op,
      left: left,
      right: right,
    }
  }
}

// ----------------------------------------------
pub struct BitIter<'a, I> {
  raw: Iter<'a, Bitpack32>,
  shape: I,
}

impl<'a, I> Clone for BitIter<'a, I>
  where I: PartialEq + Clone
{
  fn clone(&self) -> Self {
    BitIter::new(self.raw.clone(), self.shape.clone())
  }
}

impl<'a, I> Iterator for BitIter<'a, I> {
  type Item = Bitpack32;
  fn next(&mut self) -> Option<Self::Item> {
    match self.raw.next() {
      Some(x) => Some(x.clone()),
      _ => None,
    }
  }
}

impl<'a, I> BitIterator for BitIter<'a, I>
  where I: PartialEq + Clone
{
  type Index = I;

  fn shape(&self) -> Self::Index {
    self.shape.clone()
  }
}

impl<'a, I> BitIter<'a, I> {
  pub fn new(iter: Iter<Bitpack32>, shape: I) -> BitIter<I> {
    BitIter {
      raw: iter,
      shape: shape,
    }
  }
}
