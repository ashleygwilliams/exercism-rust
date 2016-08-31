pub fn reply(input: &str) -> &str {
  if input == "" {
    "Fine. Be that way!"
  } else {
    if is_question(input) {
      "Sure."
    } else if is_shout(input) {
      "Whoa, chill out!"
    }
    else {
      "Whatever."
    }
  }
}

fn is_question(input: &str) -> bool {
  input.to_string().pop().unwrap() == '?'
}

fn is_shout(input: &str) -> bool {
  input.rfind(char::is_lowercase).is_none()
}
