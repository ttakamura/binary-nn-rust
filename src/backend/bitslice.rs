use std::slice::Iter;
use std::slice::IterMut;
use backend::bitpack::Bitpack32;
use backend::bitpack::Bitpack;
use backend::bitmatrix::*;

#[derive(Debug)]
pub struct BitSlice<'a> {
  storage: &'a mut [Bitpack32],
  nbits: usize,
}

impl<'a> BitMatrix for BitSlice<'a> {
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
  fn get_block(&self, index: usize) -> &Bitpack32 {
    &self.storage[index]
  }
}

impl<'a> BitMatrixMut for BitSlice<'a> {
  #[inline]
  fn get_mut_iter(&mut self) -> IterMut<Bitpack32> {
    self.storage.iter_mut()
  }

  #[inline]
  fn get_mut_block(&mut self, index: usize) -> &mut Bitpack32 {
    &mut self.storage[index]
  }
}

impl<'a> BitSlice<'a> {
  pub fn new(slice: &mut [Bitpack32], nbits: usize) -> BitSlice {
    return BitSlice {
      storage: slice,
      nbits: nbits,
    };
  }
}

#[test]
fn offset_of_test() {
  let mut v = vec![Bitpack32::falses(), Bitpack32::falses()];
  let x = BitSlice::new(v.as_mut_slice(), 40);
  assert_eq!(x.offset_of(0), (0, 0));
  assert_eq!(x.offset_of(1), (0, 1));
  assert_eq!(x.offset_of(31), (0, 31));
  assert_eq!(x.offset_of(32), (1, 0));
  assert_eq!(x.offset_of(34), (1, 2));
  assert_eq!(x.offset_of(39), (1, 7));
}
