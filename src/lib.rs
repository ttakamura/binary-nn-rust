pub mod sandbox {
  pub fn add(x: i32, y: i32) -> i32 {
    x + y
  }


}

#[cfg(test)]
mod sandbox_test {
  use sandbox::*;

  #[test]
  fn add_test() {
    assert!(add(3, 5) == 8);
  }
}
