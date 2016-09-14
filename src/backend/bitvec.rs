const U32_SIZE: usize = 32;

pub struct BitVec {
  storage: Vec<u32>,
  nbits: usize,
}

impl BitVec {
  pub fn new(vec: Vec<u32>, nbits: usize) -> BitVec {
    return BitVec {
      storage: vec,
      nbits: nbits,
    };
  }

  pub fn get(&self, i: usize) -> Option<bool> {
    if i >= self.nbits {
      return None;
    }
    let (w, b) = self.index_of(i);
    let x = self.storage.get(w).unwrap();
    return Some((x & (1 << b)) != 0);
  }

  #[inline]
  pub fn index_of(&self, i: usize) -> (usize, usize) {
    if i >= self.nbits {
      panic!("index is over than self.nbits");
    }
    let w: usize = i / U32_SIZE;
    let b: usize = 31 - (i % U32_SIZE);
    return (w, b);
  }
}
