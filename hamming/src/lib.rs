pub fn hamming_distance(strand1: &str, strand2: &str) -> Result<u64, &'static str> {
  if strand1 == strand2 {
    Ok(0)
  } else if char_count(strand1) == char_count(strand2) {
    Ok(calculate(strand1, strand2))
  } else {
    Err("Strands must be the same length.")
  }
}

fn char_count(strand: &str) -> usize {
  strand.to_string().chars().count()
}

fn calculate(strand1: &str, strand2: &str) -> u64 {
  let mut difference = 0;
  let mut index = strand1.chars().count() - 1;
  let mut done = false;
  while !done {
    if nth_char(strand1, index) != nth_char(strand2, index) {
      difference += 1;
    }
    if index == 0 {
      done = true;
    } else {
      index -= 1;
    }
  }
  difference
}

fn nth_char(strand: &str, index: usize) -> char {
  strand.chars().nth(index).unwrap()
}
