
use std::io;
use std::collections::HashMap;

enum Rule {
  Text(String),
  RuleList(Vec<i32>),
  OneOfRuleLists(Vec<i32>, Vec<i32>)
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
  let mut count = 0;
  let mut rules: HashMap<i32, Rule> = HashMap::new();
  let mut finish_parse_rules = false;

  for line in lines {
    if !finish_parse_rules && line.find(":").is_some() {
      // Parse rules
      let (id, rule) = parse_rule(&line);
      rules.insert(id, rule);

    } else {
      finish_parse_rules = true;

      // Check if they match the rules
      if check(&rules, &line) {
        count += 1;
      }
    }
  }

  count
}

fn parse_rule(line: &str) -> (i32, Rule) {
  let mut tokens = line.split(":");
  let id: i32 = tokens
    .next()
    .unwrap()
    .trim()
    .parse()
    .unwrap();
  let rule_text = tokens.next().unwrap().trim();

  if rule_text.find("\"").is_some() {
    (id, parse_text_rule(rule_text))
  } else if rule_text.find("|").is_some() {
    let mut tokens = rule_text.split("|");
    let left = parse_rule_list(tokens.next().unwrap().trim());
    let right = parse_rule_list(tokens.next().unwrap().trim());
    (id, Rule::OneOfRuleLists(left, right))
  } else {
    (id, Rule::RuleList(parse_rule_list(rule_text)))
  }
}

fn parse_text_rule(input: &str) -> Rule {
  Rule::Text(input.split("\"").nth(1).unwrap().to_owned())
}

fn parse_rule_list(input: &str) -> Vec<i32> {
  input.split(" ").map(|s| s.trim().parse().unwrap()).collect()
}

fn check(
  rules: &HashMap<i32, Rule>,
  test_string: &str,
) -> bool {
  if let Some(len) = check_rule(&rules, &test_string, 0) {
    len == test_string.len()
  } else {
    false
  }
}

fn check_rule(
  rules: &HashMap<i32, Rule>,
  test_string: &str,
  rule_id: i32
) -> Option<usize> {
  match rules.get(&rule_id).unwrap() {
    Rule::RuleList(list) => {
      check_rule_list(rules, list, test_string)
    },
    Rule::OneOfRuleLists(list_a, list_b) => {
      check_rule_list(rules, list_a, test_string)
        .or_else(|| {
          check_rule_list(rules, list_b, test_string)
        })
    },
    Rule::Text(text) => {
      if let Some(0) = test_string.find(text) {
        Some(text.len())
      } else {
        None
      }
    }
  }
}

fn check_rule_list(
  rules: &HashMap<i32, Rule>,
  rule_list: &Vec<i32>,
  test_string: &str
) -> Option<usize> {
  let mut start = 0;

  for sub_rule in rule_list {
    if let Some(checked_len) = check_rule(rules, &test_string[start..], *sub_rule) {
      start += checked_len;
    } else {
      return None;
    }
  }

  Some(start)
}

#[cfg(test)]
mod tests {
  use super::*;

  fn test_rules() -> HashMap<i32, Rule> {
    vec![
      (0, Rule::RuleList(vec![4, 1, 5])),
      (1, Rule::OneOfRuleLists(vec![2, 3], vec![3, 2])),
      (2, Rule::OneOfRuleLists(vec![4, 4], vec![5, 5])),
      (3, Rule::OneOfRuleLists(vec![4, 5], vec![5, 4])),
      (4, Rule::Text("a".to_owned())),
      (5, Rule::Text("b".to_owned())),
    ].into_iter().collect()
  }

  #[test]
  fn test1() {
    let rules = test_rules();
    assert_eq!(check(&rules, "ababbb"), true);
  }

  #[test]
  fn test2() {
    let rules = test_rules();
    assert_eq!(check(&rules, "abbbab"), true);
  }

  #[test]
  fn test3() {
    let rules = test_rules();
    assert_eq!(check(&rules, "bababa"), false);
  }

  #[test]
  fn test4() {
    let rules = test_rules();
    assert_eq!(check(&rules, "aaabbb"), false);
  }

  #[test]
  fn test5() {
    let rules = test_rules();
    assert_eq!(check(&rules, "aaaabbb"), false);
  }
}
