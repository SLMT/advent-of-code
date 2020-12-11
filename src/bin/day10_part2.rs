
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

  // Add the starting point
  numbers.push(0);

  // Sort first
  numbers.sort();

  // Count arrangement

  // Bad: too slow
  // count_distinct(&numbers[..], 0)

  // Another way: divide the list into multiple sublists
  count_distinct_dc(&numbers)
}

// dc: divide and conquer
// ignore the numbers that must exist
fn count_distinct_dc(numbers: &Vec<i32>) -> i32 {
  let mut count = 1;
  let mut start_idx = None;

  for idx in 1 .. numbers.len() {
    if start_idx.is_none() {
      if numbers[idx] - numbers[idx - 1] < 3 {
        start_idx = Some(idx);
      }
    } else if numbers[idx] - numbers[idx - 1] == 3 {
      let start = start_idx.unwrap();
      println!("{:?}", &numbers[start .. idx]);
      count *= count_distinct(&numbers[start .. idx], numbers[start - 1]);

      // Reset the range
      start_idx = None;
    }
  }

  count
}

fn count_distinct(numbers: &[i32], last_output: i32) -> i32 {
  if numbers.is_empty() {
    1
  } else {
    let mut count = 0;
    for idx in 0 .. numbers.len() {
      if numbers[idx] - last_output <= 3 {
        count += count_distinct(&numbers[(idx + 1)..], numbers[idx]);
      } else {
        break;
      }
    }
    count
  }
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
    assert_eq!(solve(&number_list), 8);
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
    assert_eq!(solve(&number_list), 19208);
  }
}
