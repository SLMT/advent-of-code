
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

  println!("{}", solve(&lines));
}

fn solve(lines: &Vec<String>) -> usize {
  let mut group: Vec<&str> = vec![];
  let mut sum = 0;

  for line in lines {
    if line.is_empty() {
      sum += count(&group);
      group.clear();
    } else {
      group.push(line);
    }
  }

  sum + count(&group)
}

fn count(group: &Vec<&str>) -> usize {
  let mut answers = HashSet::new();

  for person in group {
    for ans in person.chars() {
      answers.insert(ans);
    }
  }

  answers.len()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test1() {
    let lines: Vec<String> = vec![
      "abc",
      "",
      "a",
      "b",
      "c",
      "",
      "ab",
      "ac",
      "",
      "a",
      "a",
      "a",
      "a",
      "",
      "b",
    ].into_iter().map(|s| s.to_owned()).collect();
    assert_eq!(solve(&lines), 11);
  }
}
