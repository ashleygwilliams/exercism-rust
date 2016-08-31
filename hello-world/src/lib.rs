pub fn hello(input: Option<&str>) -> String {
  let name = match input {
    Some(input) => input.to_string(),
    None => "World".to_string(),
  };
  format!("Hello, {}!", name)
}
