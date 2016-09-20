struct Bitpack32 {
  storage: u32,
}

trait Bitpack {
  pub fn limit_index(&self) -> usize;
  pub fn from_bool(vec: Vec<bool>) -> Bitpack;
  pub fn falses(nbits: usize) -> Bitpack;
  pub fn get(&self, index: usize) -> Option<bool>;
  pub fn set_true(&mut self, index: usize);
  pub fn set_false(&mut self, index: usize);
  pub fn mut_union(&mut self, other: &Bitpack);
  pub fn mut_intersect(&mut self, other: &Bitpack);
  pub fn mut_xor(&mut self, other: &Bitpack);
}

impl Bitpack for Bitpack32 {
  #[inline]
  pub fn limit_index(&self) -> usize {
    32
  }

  pub fn from_bool(vec: Vec<bool>) -> Bitpack32 {
    let int_vec = bool_to_int_vec(vec);
    return Bitpack32{ storage: int_vec };
  }

  pub fn falses(nbits: usize) -> Bitpack32 {
    let tmp: Vec<bool> = vec![false; nbits];
    return Bitpack32::from_bool(tmp);
  }

  pub fn get(&self, index: usize) -> Option<bool> {
    if index >= self.limit_index() {
      return None;
    }
    return Some(bit_to_bool(self.storage, index));
  }

  pub fn set_true(&mut self, index: usize) {
    if index >= self.limit_index() {
      panic!("index should smaller than self.nbits")
    }
    self.storage |= single_bit_int(index);
  }

  pub fn set_false(&mut self, index: usize) {
    if index >= self.limit_index() {
      panic!("index should smaller than self.nbits")
    }
    self.storage &= !(single_bit_int(index));
  }

  pub fn mut_union(&mut self, other: &Bitpack32) {
    self.storage = self.storage | other.storage;
  }

  pub fn mut_intersect(&mut self, other: &Bitpack32) {
    self.storage = self.storage & other.storage;
  }

  pub fn mut_xor(&mut self, other: &Bitpack32) {
    self.storage = self.storage ^ other.storage;
  }
}

#[inline]
fn bool_to_int_vec(vec: Vec<bool>) -> Vec<u32> {
  let mut int_vec: Vec<u32> = vec![];
  for block in vec.chunks(U32_SIZE) {
    let mut tmp: u32 = 0;
    for i in 0..block.len() {
      if block[i] {
        tmp = tmp | single_bit_int(i)
      }
    }
    int_vec.push(tmp);
  }
  return int_vec;
}

#[inline]
fn bit_to_bool(x: u32, index: usize) -> bool {
  if index >= U32_SIZE {
    panic!("index should smaller than 32");
  }
  (x & single_bit_int(index)) != 0
}

#[inline]
fn single_bit_int(index: usize) -> u32 {
  (1 << (U32_SIZE - (index + 1)))
}

// -------------------------------------------------------------------------------------------------
#[test]
fn bool_to_int_vec_test() {
  assert_eq!(bool_to_int_vec(vec![true, false]), vec![1 << 31]);
  assert_eq!(bool_to_int_vec(vec![true, true, true]), vec![7 << 29]);
  assert_eq!(bool_to_int_vec(vec![false, false, true]), vec![1 << 29]);

  let mut vec: Vec<bool> = vec![];
  for i in 0..33 {
    if i < 31 {
      vec.push(false);
    } else {
      vec.push(true);
    }
  }
  assert_eq!(bool_to_int_vec(vec), vec![1, 1 << 31]);
}

#[test]
fn bit_to_bool_test() {
  assert_eq!(bit_to_bool(1 << 30, 0), false);
  assert_eq!(bit_to_bool(1 << 30, 1), true);
  assert_eq!(bit_to_bool(1 << 30, 2), false);

  assert_eq!(bit_to_bool(2, 29), false);
  assert_eq!(bit_to_bool(2, 30), true);
  assert_eq!(bit_to_bool(2, 31), false);

  assert_eq!(bit_to_bool(1, 30), false);
  assert_eq!(bit_to_bool(1, 31), true);

  assert_eq!(bit_to_bool(0, 0), false);
  assert_eq!(bit_to_bool(0, 31), false);
}

#[test]
#[should_panic(expected = "index should smaller than 32")]
fn bit_to_bool_panic_test() {
  assert!(bit_to_bool(0, 32));
}
