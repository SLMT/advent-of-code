
use std::io;

fn main() {
  let mut input = String::new();
  let mut lines = vec![];

  loop {
    // Read inputs
    input.clear();
    let read_count = io::stdin().read_line(&mut input).unwrap();
    if read_count == 0 { // EOF
      break;
    }

    // Add to lines
    lines.push(input.trim().to_owned());
  }

  println!("{}", solve(&lines));
}

fn solve(lines: &Vec<String>) -> usize {
  let mut valid_count = 0;

  for line in lines {
    if is_valid(line) {
      valid_count += 1;
    }
  }
  
  valid_count
}

fn is_valid(line: &String) -> bool {
  // Tokenization
  let mut tokens = line.split(" ");
  let positions = tokens.next().unwrap();
  let letter = tokens.next().unwrap().chars().next().unwrap();
  let password = tokens.next().unwrap();

  // Get the positions
  let mut position_tokens = positions.split("-");
  let first_pos: usize = position_tokens.next().unwrap().parse().unwrap();
  let second_pos: usize = position_tokens.next().unwrap().parse().unwrap();

  // Matches
  let does_first_match =
    password.chars().nth(first_pos - 1).unwrap() == letter;
  let does_second_match =
    password.chars().nth(second_pos - 1).unwrap() == letter;
  
  // Case branches
  if does_first_match {
    if does_second_match {
      false
    } else {
      true
    }
  } else {
    if does_second_match {
      true
    } else {
      false
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test1() {
    let lines = vec![
      "1-3 a: abcde".to_owned(),
      "1-3 b: cdefg".to_owned(),
      "2-9 c: ccccccccc".to_owned(),
    ];
    assert_eq!(solve(&lines), 1);
  }
}
