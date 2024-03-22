pub fn hello() -> &'static str {
  "hello world"
}

#[test]
#[cfg(test)]
fn test_1() {
  assert_eq!("hello world", hello());
}
