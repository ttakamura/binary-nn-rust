use std::slice::Iter;
use std::slice::IterMut;
use backend::bitpack::Bitpack32;
use backend::bitpack::Bitpack;
use backend::bitmatrix::*;
use backend::bititer::*;

#[derive(Debug)]
pub struct BitVec {
  storage: Vec<Bitpack32>,
  nbits: usize,
}

impl BitMatrix for BitVec {
  type Index = usize;

  fn offset_of(&self, index: Self::Index) -> (usize, usize) {
    if index >= self.nbits {
      panic!("index should smaller than self.nbits")
    }
    let w: usize = index / Bitpack32::limit_index();
    let b: usize = index % Bitpack32::limit_index();
    return (w, b);
  }

  fn len(&self) -> Self::Index {
    self.nbits
  }

  fn block_len(&self) -> usize {
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
  pub fn new(vec: Vec<Bitpack32>, nbits: usize) -> BitVec {
    return BitVec {
      storage: vec,
      nbits: nbits,
    };
  }

  pub fn falses(nbits: <BitVec as BitMatrix>::Index) -> Self {
    let mut vec: Vec<Bitpack32> = vec![];
    for _ in 0..BitVec::block_num_of(nbits) {
      vec.push(Bitpack32::falses());
    }
    return BitVec::new(vec, nbits);
  }

  #[inline]
  fn block_num(&self) -> usize {
    return BitVec::block_num_of(self.nbits);
  }

  #[inline]
  fn block_num_of(ncol: usize) -> usize {
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
