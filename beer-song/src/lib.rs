pub fn sing(start: u32, end: u32) -> String {
  (end..(start + 1)).rev().fold(String::new(), |mut song, n| {
    song.push_str(&verse(n));
    if n != end {
      song.push_str("\n")
    }
    song
  })
}

pub fn verse(num: u32) -> String {
  if num == 0 {
    "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string()
  } else if num == 1 {
    "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_string()
  } else if num == 2 { 
     "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n".to_string()
  } else {
    format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n",
    num,
    num,
    num-1)
  }
}
