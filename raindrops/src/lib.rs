pub fn raindrops(number: i64) -> String {
  let mut word = "".to_string();
  if number % 3 == 0 {
    word.push_str("Pling");
  }
  if number % 5 == 0 {
    word.push_str("Plang");
  }
  if number % 7 == 0 {
    word.push_str("Plong");
  };
  if word.len() == 0 {
    word = number.to_string();
  }
  word
}
