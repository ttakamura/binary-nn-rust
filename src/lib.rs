pub mod sandbox {
  pub fn add(x: i32, y: i32) -> i32 {
    x + y
  }
}

// pub mod bnn {
//   pub struct BitVec {
//     storage: Vec<u32>,
//     nbits: usize,
//     unit_size: usize
//   }
//
//   pub fn get(&self, i: usize) -> Option<bool> {
//     if i >= self.nbits {
//       return None;
//     }
//     let w = i / self.unit_size;
//     let b = i % self.unit_size;
//     let x = self.storage.get(w).unwrap();
//     return (x & (1 << b)) != 0;
//   }
// }
