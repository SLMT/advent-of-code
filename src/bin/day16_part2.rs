
use std::io;
use std::collections::{HashSet, HashMap};

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

fn solve(lines: Vec<String>) -> i64 {
  let mut line_idx = 0;
  let mut ticket_results = vec![];

  // Read rules
  let mut rules = vec![];
  while !lines[line_idx].is_empty() {
    rules.push(Rule::from_str(&lines[line_idx]));
    line_idx += 1;
  }

  // Read your ticket
  line_idx += 2;
  let your_ticket: Vec<i32> = lines[line_idx]
      .split(",")
      .map(|s| s.trim().parse().unwrap())
      .collect();
  line_idx += 1;

  // Read other tickets
  line_idx += 2;
  while line_idx < lines.len() {
    let ticket: Vec<i32> = lines[line_idx]
      .split(",")
      .map(|s| s.trim().parse().unwrap())
      .collect();
    
    if let Some(possible_fields) = find_possible_fields(&ticket, &rules) {
      ticket_results.push(possible_fields);
    }

    line_idx += 1;
  }

  // Find the position of fields
  let field_to_pos = find_positions_mapping(&ticket_results);

  // Multiply the values of the fields that starts with 'departure'
  let mut result: i64 = 1;
  for (field, pos) in field_to_pos {
    if field.starts_with("departure") {
      result *= your_ticket[pos] as i64;
    }
  }

  result
}

fn find_possible_fields(ticket: &Vec<i32>, rules: &Vec<Rule>) -> Option<Vec<HashSet<String>>> {
  let mut possible_fields = vec![];

  for value in ticket {
    let mut valid_rules = HashSet::new();
    for rule in rules {
      if rule.is_valid(*value) {
        valid_rules.insert(rule.name.clone());
      }
    }
    
    // Empty -> invalid ticket
    if valid_rules.is_empty() {
      return None;
    }

    possible_fields.push(valid_rules);
  }

  Some(possible_fields)
}

fn find_positions_mapping(tickets: &Vec<Vec<HashSet<String>>>)
  -> HashMap<String, usize> {
  let mut field_to_pos = HashMap::new();
  let field_count = tickets[0].len();
  let mut found_pos = HashSet::new();

  while found_pos.len() < field_count {
    // Check each field
    for field_pos in 0..field_count {
      // Skip the known fields
      if found_pos.contains(&field_pos) {
        continue;
      }

      // Find the intersection of possible fields for all tickets
      let mut possible_set = tickets[0][field_pos].clone();
      for ticket_id in 1..tickets.len() {
        possible_set = possible_set
          .intersection(&tickets[ticket_id][field_pos])
          .map(|s| s.to_owned())
          .collect();
      }

      // Remove the known fields
      for field in field_to_pos.keys() {
        possible_set.remove(field);
      }

      // If the size is one, bingo!
      if possible_set.len() == 1 {
        let field = possible_set.into_iter().next().unwrap();
        field_to_pos.insert(field, field_pos);
        found_pos.insert(field_pos);
      } else if possible_set.len() == 0 {
        panic!("Something wrong! Coludn't find a intersection for field #{}",
          field_pos);
      }
    }
  }

  field_to_pos
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test1() {
    let lines: Vec<String> = vec![
      "departure class: 0-1 or 4-19",
      "row: 0-5 or 8-19",
      "seat: 0-13 or 16-19",
      "",
      "your ticket:",
      "11,12,13",
      "",
      "nearby tickets:",
      "3,9,18",
      "15,1,5",
      "5,14,9",
    ].into_iter().map(|s| s.to_owned()).collect();
    assert_eq!(solve(lines), 12);
  }
}
