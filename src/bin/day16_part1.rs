
use std::io;

struct Rule {
  name: String,
  ranges: Vec<(i32, i32)>
}

impl Rule {
  fn from_str(string: &str) -> Rule {
    let mut tokens = string.trim().split(":");
    let name = tokens.next().unwrap().trim().to_owned();

    let mut ranges = vec![];
    for rule_str in tokens.next().unwrap().split("or") {
      let mut rule_tokens = rule_str.trim().split("-");
      let lower: i32 = rule_tokens.next().unwrap().parse().unwrap();
      let upper: i32 = rule_tokens.next().unwrap().parse().unwrap();
      ranges.push((lower, upper));
    }

    Rule {
      name,
      ranges
    }
  }

  fn is_valid(&self, value: i32) -> bool {
    for (lower, upper) in &self.ranges {
      if value >= *lower && value <= *upper {
        return true;
      }
    }

    false
  }
}

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

  println!("{}", solve(lines));
}

fn solve(lines: Vec<String>) -> i32 {
  let mut line_idx = 0;
  let mut invalid_sum = 0;

  // Read rules
  let mut rules = vec![];
  while !lines[line_idx].is_empty() {
    rules.push(Rule::from_str(&lines[line_idx]));
    line_idx += 1;
  }

  // Ignore your ticket
  line_idx += 1;
  while !lines[line_idx].is_empty() {
    line_idx += 1;
  }

  // Read other tickets
  line_idx += 2;
  while line_idx < lines.len() {
    let ticket: Vec<i32> = lines[line_idx]
      .split(",")
      .map(|s| s.trim().parse().unwrap())
      .collect();
    
    invalid_sum += sum_invalid_values(&ticket, &rules);

    line_idx += 1;
  }

  invalid_sum
}

fn sum_invalid_values(ticket: &Vec<i32>, rules: &Vec<Rule>) -> i32 {
  let mut sum = 0;
  'val_loop: for value in ticket {
    for rule in rules {
      if rule.is_valid(*value) {
        continue 'val_loop;
      }
    }
    sum += value;
  }
  sum
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test1() {
    let lines: Vec<String> = vec![
      "class: 1-3 or 5-7",
      "row: 6-11 or 33-44",
      "seat: 13-40 or 45-50",
      "",
      "your ticket:",
      "7,1,14",
      "",
      "nearby tickets:",
      "7,3,47",
      "40,4,50",
      "55,2,20",
      "38,6,12",
    ].into_iter().map(|s| s.to_owned()).collect();
    assert_eq!(solve(lines), 71);
  }
}
