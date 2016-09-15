const U32_SIZE: usize = 32;

pub struct BitVec {
  storage: Vec<u32>,
  nbits: usize,
}

fn mask_for_bits(bits: usize) -> u32 {
    // Note especially that a perfect multiple of u32::BITS should mask all 1s.
    !0 >> (u32::BITS - bits % u32::BITS) % u32::BITS
}

impl BitVec {
  pub fn new(vec: Vec<u32>, nbits: usize) -> BitVec {
    return BitVec {
      storage: vec,
      nbits: nbits,
    };
  }

  pub fn new_bool(vec: Vec<bool>) -> BitVec {
    let nbits = vec.len();
    let mut int_vec = Vec::u32::new();

    for block in vec.chunks(U32_SIZE) {
      let mut tmp: u32 = 0;
      for i in 1..block.len() {
        tmp = tmp | (1 << U32_SIZE - (i + 1))
      }
    }

    return BitVec {
      storage: int_vec,
      nbits: nbits,
    };
  }

  pub fn get(&self, i: usize) -> Option<bool> {
    if i >= self.nbits {
      return None;
    }
    let (w, b) = BitVec::offset_of(i);
    let x = self.storage.get(w).unwrap();
    return Some((x & (1 << b)) != 0);
  }

  #[inline]
  fn offset_of(i: usize) -> (usize, usize) {
    let w: usize = i / U32_SIZE;
    let b: usize = U32_SIZE - ((i % U32_SIZE) + 1);
    return (w, b);
  }

  #[inline]
  fn bitmask()
}
