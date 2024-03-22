pub fn hello() -> &'static str {
  "hello world"
}

#[cfg(test)]
mod tests {
  use super::hello;

  #[test]
  fn test_1() {
    assert_eq!("hello world", hello());
  }
}
