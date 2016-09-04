use std::collections::HashMap;

pub fn nucleotide_counts(strand: &str) -> HashMap<char, usize> {
  let mut result = HashMap::new();
  let nucleotides = ['A', 'T', 'C', 'G'];
  for &n in nucleotides.into_iter() {
    result.insert(n, count(n, strand));
  }
  result
}

pub fn count(nucleotide: char, strand: &str) -> usize {
  strand.chars().filter( |&c| c == nucleotide ).collect::<Vec<char>>().len()
}
