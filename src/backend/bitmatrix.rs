const U32_SIZE: usize = 32;

pub struct BitMatrix {
  storage: Vec<BitVec>,
  row_major: bool,
}
