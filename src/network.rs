extern crate bincode;
extern crate serde;

use std::fs::File;
use std::error::Error;

pub trait Network
  where Self: serde::Serialize,
        Self: serde::Deserialize,
        Self: Sized
{
  fn serialize_into(&self, path: String) {
    let mut file = match File::create(&path) {
      Ok(file) => file,
      Err(why) => panic!("Couldn't create {}, {}", path, Error::description(&why)),
    };
    match bincode::serialize_into(&mut file, &self, bincode::Infinite) {
      Ok(_) => {}
      Err(why) => panic!("Couldn't serialize {}, {}", path, Error::description(&why)),
    };
  }

  fn deserialize_from(path: String) -> Self {
    let mut file = match File::open(&path) {
      Ok(file) => file,
      Err(why) => panic!("Couldn't open {}, {}", path, Error::description(&why)),
    };
    return match bincode::deserialize_from(&mut file, bincode::Infinite) {
      Ok(net) => net,
      Err(why) => {
        panic!("Couldn't deserialize {}, {}",
               path,
               Error::description(&why))
      }
    };
  }
}
