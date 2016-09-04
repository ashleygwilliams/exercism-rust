use std::collections::HashMap;

pub fn word_count(text: &str) -> HashMap<String, u32> {
  let mut result = HashMap::new();
  let charsets = text.split( |c| c == ' ' || c == '_' || c == ',');
  for charset in charsets {
      let word = clean(charset);
      if word.is_some() {
        let counter = result.entry(word.unwrap()).or_insert(0);
        *counter += 1;
      }
  }
  result
}

fn clean(charset: &str) -> Option<String> {
  let mut word = String::new();
  for c in charset.chars() {
    if c.is_digit(36) {
      word.push(c.to_lowercase().next().unwrap());
    }
  }
  if word.len() !=  0 {
    return Some(word);
  } else {
    return None;
  }
}
