pub fn hello() -> &'static str {
  "hello world"
}

#[cfg(test)]
mod tests {
  use super::hello;
  use another_lib::{
    try_hello,
    try_world,
  };

  #[test]
  fn test_1() {
    assert_eq!("hello world", hello());
  }

  #[test]
  fn test_2() {
    assert_eq!("hello", try_hello().unwrap());
  }

  #[test]
  fn test_3() {
    assert_eq!("world", try_world().unwrap());
  }
}
