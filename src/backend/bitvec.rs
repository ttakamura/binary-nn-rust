use std::slice::Iter;
use std::slice::IterMut;
use backend::bitpack::Bitpack32;
use backend::bitpack::Bitpack;
use backend::bitmatrix::BitMatrix;

#[derive(Debug)]
pub struct BitVec {
  storage: Vec<Bitpack32>,
  nbits: usize,
}

impl BitMatrix for BitVec {
  type Index = usize;

  #[inline]
  fn offset_of(&self, index: Self::Index) -> (usize, usize) {
    if index >= self.nbits {
      panic!("index should smaller than self.nbits")
    }
    let w: usize = index / Bitpack32::limit_index();
    let b: usize = index % Bitpack32::limit_index();
    return (w, b);
  }

  #[inline]
  fn len(&self) -> Self::Index {
    self.nbits
  }

  #[inline]
  fn get_iter(&self) -> Iter<Bitpack32> {
    self.storage.iter()
  }

  #[inline]
  fn get_mut_iter(&mut self) -> IterMut<Bitpack32> {
    self.storage.iter_mut()
  }

  #[inline]
  fn get_block(&self, index: usize) -> &Bitpack32 {
    &self.storage[index]
  }

  #[inline]
  fn get_mut_block(&mut self, index: usize) -> &mut Bitpack32 {
    &mut self.storage[index]
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
    let block_num = nbits / Bitpack32::limit_index() + 1;
    let mut vec: Vec<Bitpack32> = vec![];
    for _ in 0..block_num {
      vec.push(Bitpack32::falses());
    }
    return BitVec::new(vec, nbits);
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
