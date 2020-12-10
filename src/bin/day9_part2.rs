
use std::io;
use std::collections::HashSet;

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

  println!("{}", solve(&number_list, 25));
}

fn solve(number_list: &Vec<i64>, preamble_size: usize) -> i64 {
  let critical_number = find_critical_number(number_list, preamble_size);
  let matched_list = search_match_list(number_list, critical_number);
  matched_list.iter().max().unwrap() + matched_list.iter().min().unwrap()
}

fn find_critical_number(number_list: &Vec<i64>, preamble_size: usize) -> i64 {
  let mut preamble: HashSet<i64> = HashSet::new();

  for (idx, number) in number_list.iter().enumerate() {
    // First {preamble_size} numbers go to the set
    if idx < preamble_size {
      preamble.insert(*number);
    } else {
      // Check the rule
      let mut check_pass = false;

      for check_idx in (idx - preamble_size) .. idx {
        let complement = number - number_list[check_idx];
        if preamble.contains(&complement) {
          check_pass = true;
          break;
        }
      }

      if !check_pass {
        return *number;
      }

      preamble.remove(&number_list[idx - preamble_size]);
      preamble.insert(*number);
    }
  }

  panic!("Cannot find any number");
}

fn search_match_list(number_list: &Vec<i64>, invalid_number: i64) -> &[i64] {
  for start_idx in 0 .. number_list.len() {
    let mut sum = 0;
    for end_idx in start_idx .. number_list.len() {
      sum += number_list[end_idx];
      if sum == invalid_number {
        return &number_list[start_idx .. (end_idx + 1)];
      } else if sum > invalid_number {
        break;
      }
    }
  }

  panic!("Cannot find any matched list");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test1() {
    let number_list: Vec<i64> = vec![
      35,
      20,
      15,
      25,
      47,
      40,
      62,
      55,
      65,
      95,
      102,
      117,
      150,
      182,
      127,
      219,
      299,
      277,
      309,
      576,
    ];
    assert_eq!(solve(&number_list, 5), 62);
  }
}
