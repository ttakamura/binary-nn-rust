use std::iter::FromIterator;
use backend::bitpack::Bitpack32;
use backend::bitpack::Bitpack;
use backend::bitmatrix_trait::*;
use backend::bititer::*;

#[derive(Debug)]
pub struct BitVec {
  storage: Vec<Bitpack32>,
  nbits: u32,
}

impl BitMatrix for BitVec {
  type Index = u32;

  fn new(vec: Vec<Bitpack32>, nbits: Self::Index) -> BitVec {
    return BitVec {
      storage: vec,
      nbits: nbits,
    };
  }

  fn offset_of(&self, index: Self::Index) -> (u32, u32) {
    if index >= self.nbits {
      panic!("index should smaller than self.nbits")
    }
    let w = index / Bitpack32::limit_index();
    let b = index % Bitpack32::limit_index();
    return (w, b);
  }

  fn len(&self) -> Self::Index {
    self.nbits
  }

  fn block_len(&self) -> u32 {
    self.block_num()
  }

  fn as_slice(&self) -> &[Bitpack32] {
    self.storage.as_slice()
  }
}

impl BitMatrixMut for BitVec {
  fn as_mut_slice(&mut self) -> &mut [Bitpack32] {
    self.storage.as_mut_slice()
  }
}

impl BitVec {
  pub fn falses(nbits: <BitVec as BitMatrix>::Index) -> Self {
    let mut vec: Vec<Bitpack32> = vec![];
    for _ in 0..BitVec::block_num_of(nbits) {
      vec.push(Bitpack32::falses());
    }
    return BitVec::new(vec, nbits);
  }

  pub fn from_iter<I>(iter: I) -> Self
    where I: BitIterator<Item = Bitpack32, Index = u32>
  {
    let nbits = iter.shape().clone();
    Self::new(Vec::from_iter(iter), nbits)
  }

  pub fn iter(&self) -> BitIter<u32> {
    return BitIter::new(self.as_slice().into_iter(), self.nbits);
  }

  #[inline]
  fn block_num(&self) -> u32 {
    return BitVec::block_num_of(self.nbits);
  }

  #[inline]
  fn block_num_of(ncol: u32) -> u32 {
    return ncol / Bitpack32::limit_index() + 1;
  }
}

// -------------------------------------------------------------------------------------------------
#[test]
fn offset_of_test() {
  let x = BitVec::new(vec![Bitpack32::falses(), Bitpack32::falses()], 40);
  assert_eq!(x.offset_of(0), (0, 0));
  assert_eq!(x.offset_of(1), (0, 1));
  assert_eq!(x.offset_of(31), (0, 31));
  assert_eq!(x.offset_of(32), (1, 0));
  assert_eq!(x.offset_of(34), (1, 2));
  assert_eq!(x.offset_of(39), (1, 7));
}
