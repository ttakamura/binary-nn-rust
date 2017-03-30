pub trait Layer {
  fn encode(&self) -> Vec<u8>;
  fn decode(vec: Vec<u8>) -> Self;
}
