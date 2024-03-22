use anyhow::Error;

pub fn try_hello() -> Result<&'static str, Error> {
  Ok("hello")
}

pub fn try_world() -> Result<&'static str, Error> {
  Ok("world")
}
