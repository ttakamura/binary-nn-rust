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
    let w = i / U32_SIZE;
    let b = i % U32_SIZE;
    let x = self.storage.get(w).unwrap();
    return Some((x & (1 << b)) != 0);
  }
}
