
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
  let limits = tokens.next().unwrap();
  let letter = tokens.next().unwrap().chars().next().unwrap();
  let password = tokens.next().unwrap();

  // Get the bound
  let mut limit_tokens = limits.split("-");
  let lower_bound: usize = limit_tokens.next().unwrap().parse().unwrap();
  let upper_bound: usize = limit_tokens.next().unwrap().parse().unwrap();

  // Count
  let mut count = 0;
  for char in password.chars() {
    if char == letter {
      count += 1;
    }
  }

  count >= lower_bound && count <= upper_bound
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
    assert_eq!(solve(&lines), 2);
  }
}
