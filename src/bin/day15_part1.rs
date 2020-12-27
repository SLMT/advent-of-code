
use std::io;
use std::collections::HashMap;

fn main() {
  let mut input = String::new();
  let mut lines = vec![];

  // Read inputs
  loop {
    input.clear();
    let read_count = io::stdin().read_line(&mut input).unwrap();
    if read_count == 0 { // EOF
      break;
    }
    lines.push(input.trim().to_owned());
  }

  // Parse to numbers
  let number_list = lines[0]
    .split(",")
    .map(|s| s.trim().parse().unwrap())
    .collect();

  println!("{}", solve(number_list));
}

fn solve(number_list: Vec<i32>) -> i32 {
  let mut spoken_number = number_list[1];
  let mut last_spoken_number = number_list[0];
  let mut last_spoken_turn = HashMap::new();

  for turn in 2..=2020 {
    if turn <= number_list.len() {
      spoken_number = number_list[turn - 1];
    } else {
      if let Some(last_turn) = last_spoken_turn.get(&last_spoken_number) {
        spoken_number = ((turn - 1) - *last_turn) as i32;
      } else {
        spoken_number = 0;
      }
    }
    last_spoken_turn.insert(last_spoken_number, turn - 1);
    last_spoken_number = spoken_number;
  }

  spoken_number
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test1() {
    let number_list: Vec<i32> = vec![0,3,6];
    assert_eq!(solve(number_list), 436);
  }

  #[test]
  fn test2() {
    let number_list: Vec<i32> = vec![1,3,2];
    assert_eq!(solve(number_list), 1);
  }

  #[test]
  fn test3() {
    let number_list: Vec<i32> = vec![2,1,3];
    assert_eq!(solve(number_list), 10);
  }

  #[test]
  fn test4() {
    let number_list: Vec<i32> = vec![1,2,3];
    assert_eq!(solve(number_list), 27);
  }

  #[test]
  fn test5() {
    let number_list: Vec<i32> = vec![2,3,1];
    assert_eq!(solve(number_list), 78);
  }

  #[test]
  fn test6() {
    let number_list: Vec<i32> = vec![3,2,1];
    assert_eq!(solve(number_list), 438);
  }

  #[test]
  fn test7() {
    let number_list: Vec<i32> = vec![3,1,2];
    assert_eq!(solve(number_list), 1836);
  }
}
