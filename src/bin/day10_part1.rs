
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

  // Parse to numbers
  let number_list = lines.iter().map(|s| s.parse().unwrap()).collect();

  println!("{}", solve(&number_list));
}

fn solve(number_list: &Vec<i32>) -> i32 {
  let mut numbers = number_list.clone();

  // Sort first
  numbers.sort();

  // Check difference
  let mut diffs = [0; 3];
  let mut last_number = 0;
  for number in numbers {
    let diff = (number - last_number) as usize;
    diffs[diff - 1] += 1;
    last_number = number;
  }

  diffs[0] * (diffs[2] + 1)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test1() {
    let number_list: Vec<i32> = vec![
      16,
      10,
      15,
      5,
      1,
      11,
      7,
      19,
      6,
      12,
      4,
    ];
    assert_eq!(solve(&number_list), 35);
  }

  #[test]
  fn test2() {
    let number_list: Vec<i32> = vec![
      28,
      33,
      18,
      42,
      31,
      14,
      46,
      20,
      48,
      47,
      24,
      23,
      49,
      45,
      19,
      38,
      39,
      11,
      1,
      32,
      25,
      35,
      8,
      17,
      7,
      9,
      4,
      2,
      34,
      10,
      3,
    ];
    assert_eq!(solve(&number_list), 220);
  }
}
