struct Bitpack32 {
  storage: u32,
}

trait Bitpack {
  pub fn limit_index(&self) -> usize;
  pub fn from_bool(vec: Vec<bool>) -> Bitpack;
  pub fn falses() -> Bitpack;
  pub fn get(&self, index: usize) -> Option<bool>;
  pub fn set_true(&mut self, index: usize);
  pub fn set_false(&mut self, index: usize);
  pub fn mut_union(&mut self, other: &Bitpack);
  pub fn mut_intersect(&mut self, other: &Bitpack);
  pub fn mut_xor(&mut self, other: &Bitpack);
}

impl Bitpack for Bitpack32 {
  #[inline]
  pub fn limit_index() -> usize {
    32
  }

  pub fn from_bool(vec: Vec<bool>) -> Bitpack32 {
    let val = bool_to_int(vec);
    return Bitpack32{ storage: val };
  }

  pub fn falses() -> Bitpack32 {
    let tmp: Vec<bool> = vec![false; Bitpack32::limit_index()];
    return Bitpack32::from_bool(tmp);
  }

  pub fn get(&self, index: usize) -> Option<bool> {
    if index >= Bitpack32::limit_index() {
      panic!("index should smaller than limit_index")
    }
    return Some(bit_to_bool(self.storage, index));
  }

  pub fn set_true(&mut self, index: usize) {
    if index >= Bitpack32::limit_index() {
      panic!("index should smaller than limit_index")
    }
    self.storage |= single_bit_int(index);
  }

  pub fn set_false(&mut self, index: usize) {
    if index >= Bitpack32::limit_index() {
      panic!("index should smaller than limit_index")
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
fn bool_to_int(vec: Vec<bool>) -> u32 {
  let mut tmp: u32 = 0;
  for i in 0..Bitpack32::limit_index() {
    if vec[i] {
      tmp = tmp | single_bit_int(i)
    }
  }
  return tmp;
}

#[inline]
fn bit_to_bool(x: u32, index: usize) -> bool {
  if index >= Bitpack32::limit_index() {
    panic!("index should smaller than 32");
  }
  (x & single_bit_int(index)) != 0
}

#[inline]
fn single_bit_int(index: usize) -> u32 {
  (1 << (Bitpack32::limit_index() - (index + 1)))
}

// -------------------------------------------------------------------------------------------------
#[test]
fn bool_to_int_test() {
  assert_eq!(bool_to_int(vec![true, false]), 1 << 31);
  assert_eq!(bool_to_int(vec![true, true, true]), 7 << 29);
  assert_eq!(bool_to_int(vec![false, false, true]), 1 << 29);
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
