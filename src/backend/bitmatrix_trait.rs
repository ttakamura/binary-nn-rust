use std::cmp::PartialEq;
use backend::bitpack::Bitpack32;
use backend::bitpack::Bitpack;
use backend::bititer::*;

pub trait BitMatrix {
  type Index: PartialEq;

  fn get(&self, index: Self::Index) -> bool {
    let (w, b) = self.offset_of(index);
    return self.block(w).get(b);
  }

  fn iter(&self) -> BitIter {
    BitIter::new(self.as_slice(), 0, self.block_len(), 1, 1)
  }

  fn block(&self, index: usize) -> &Bitpack32 {
    &self.as_slice()[index]
  }

  fn offset_of(&self, index: Self::Index) -> (usize, usize);
  fn len(&self) -> Self::Index;
  fn block_len(&self) -> usize;
  fn as_slice(&self) -> &[Bitpack32];
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

  fn mut_iter(&mut self) -> BitIterMut {
    let length = self.block_len();
    return BitIterMut::new(self.as_mut_slice(), 0, length, 1, 1);
  }

  fn mut_block(&mut self, index: usize) -> &mut Bitpack32 {
    &mut self.as_mut_slice()[index]
  }

  fn as_mut_slice(&mut self) -> &mut [Bitpack32];
}
