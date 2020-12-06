
use std::io;

fn main() {
  let mut input = String::new();
  let mut lines = vec![];

  // Read input as lines
  loop {
    input.clear();
    let read_count = io::stdin().read_line(&mut input).unwrap();
    if read_count == 0 { // EOF
      break;
    }
    lines.push(input.trim().to_owned());
  }

  println!("{}", solve(&lines));
}

fn solve(lines: &Vec<String>) -> i32 {
  lines.iter()
    .map(|s| to_seat_id(s))
    .max()
    .unwrap()
}

fn to_seat_id(word: &str) -> i32 {
  let row_digits = &word[..7]
    .replace('B', "1")
    .replace('F', "0");
  let col_digits = &word[7..]
    .replace('R', "1")
    .replace('L', "0");

  let row_id = i32::from_str_radix(row_digits, 2).unwrap();
  let col_id = i32::from_str_radix(col_digits, 2).unwrap();

  row_id * 8 + col_id
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test1() {
    let lines: Vec<String> = vec![
      "BFFFBBFRRR",
      "FFFBBBFRRR",
      "BBFFBBFRLL",
    ].into_iter().map(|s| s.to_owned()).collect();
    assert_eq!(solve(&lines), 820);
  }
}
