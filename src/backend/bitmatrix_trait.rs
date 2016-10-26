use std::cmp::PartialEq;
use backend::bitpack::Bitpack32;
use backend::bitpack::Bitpack;
use backend::bititer::*;

pub trait BitMatrix
  where Self: Sized
{
  type Index: PartialEq + Clone;

  // fn union<'a>(&'a self, other: &'a Self) -> BitIterZip<BitIter, BitIter, BitOpUnion> {
  //   self.iter().union(&other.iter())
  // }
  //
  // fn xnor<'a>(&'a self, other: &'a Self) -> BitIterZip<BitIter, BitIter, BitOpXnor> {
  //   self.iter().xnor(&other.iter())
  // }

  fn get(&self, index: Self::Index) -> bool {
    let (w, b) = self.offset_of(index);
    return self.block(w).get(b);
  }

  fn block(&self, index: u32) -> &Bitpack32 {
    &self.as_slice()[index as usize]
  }

  fn offset_of(&self, index: Self::Index) -> (u32, u32);
  fn len(&self) -> Self::Index;
  fn block_len(&self) -> u32;
  fn as_slice(&self) -> &[Bitpack32];
  fn new(storage: Vec<Bitpack32>, nbits: Self::Index) -> Self;
}

pub trait BitMatrixMut: BitMatrix {
  fn set_true(&mut self, index: Self::Index) {
    let (w, b) = self.offset_of(index);
    return self.mut_block(w).set_true(b);
  }

  fn set_false(&mut self, index: Self::Index) {
    let (w, b) = self.offset_of(index);
    return self.mut_block(w).set_false(b);
  }

  fn mut_block(&mut self, index: u32) -> &mut Bitpack32 {
    &mut self.as_mut_slice()[index as usize]
  }

  fn as_mut_slice(&mut self) -> &mut [Bitpack32];
}
