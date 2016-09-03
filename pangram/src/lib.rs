pub fn is_pangram(sentence: &str) -> bool {
  let mut characters: Vec<char> = String::from(sentence).chars()
    .map( |c| c.to_uppercase().next().unwrap())
    .filter( |c| c.is_digit(36) && !c.is_numeric())
    .collect();
  characters.sort();
  characters.dedup();
  characters.len() == 26
}
