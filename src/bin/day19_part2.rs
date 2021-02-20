
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
      if !finish_parse_rules {
        finish_parse_rules = true;
        rules.insert(8, Rule::OneOfRuleLists(vec![42], vec![42, 8]));
        rules.insert(11, Rule::OneOfRuleLists(vec![42, 31], vec![42, 11, 31]));
      }

      // Check if they match the rules
      if check(&rules, &line) {
        count += 1;
        println!("Matched: {}", line);
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
  for matched_length in check_rule(&rules, &test_string, 0) {
    if matched_length == test_string.len() {
      return true;
    }
  }
  false
}

fn check_rule(
  rules: &HashMap<i32, Rule>,
  test_string: &str,
  rule_id: i32
) -> Vec<usize> {
  match rules.get(&rule_id).unwrap() {
    Rule::RuleList(list) => {
      check_rule_list(rules, list, test_string)
    },
    Rule::OneOfRuleLists(list_a, list_b) => {
      let mut matched_starts1 = check_rule_list(rules, list_a, test_string);
      let mut matched_starts2 = check_rule_list(rules, list_b, test_string);
      matched_starts1.append(&mut matched_starts2);
      matched_starts1
    },
    Rule::Text(text) => {
      if let Some(0) = test_string.find(text) {
        vec![text.len()]
      } else {
        vec![]
      }
    }
  }
}

fn check_rule_list(
  rules: &HashMap<i32, Rule>,
  rule_list: &Vec<i32>,
  test_string: &str
) -> Vec<usize> {
  let mut matched_starts = vec![0]; // starts from 0
  let mut next_starts = vec![];

  for sub_rule in rule_list {
    for start in &matched_starts {
      let matched_lengths = check_rule(rules, &test_string[*start..], *sub_rule);
      for len in matched_lengths {
        next_starts.push(start + len);
      }
    }

    // Dump
    matched_starts.clear();
    matched_starts.append(&mut next_starts);
  }

  matched_starts
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    let input = r#"42: 9 14 | 10 1
    9: 14 27 | 1 26
    10: 23 14 | 28 1
    1: "a"
    11: 42 31
    5: 1 14 | 15 1
    19: 14 1 | 14 14
    12: 24 14 | 19 1
    16: 15 1 | 14 14
    31: 14 17 | 1 13
    6: 14 14 | 1 14
    2: 1 24 | 14 4
    0: 8 11
    13: 14 3 | 1 12
    15: 1 | 14
    17: 14 2 | 1 7
    23: 25 1 | 22 14
    28: 16 1
    4: 1 1
    20: 14 14 | 1 15
    3: 5 14 | 16 1
    27: 1 6 | 14 18
    14: "b"
    21: 14 1 | 1 14
    25: 1 1 | 1 14
    22: 14 14
    8: 42
    26: 14 22 | 1 20
    18: 15 15
    7: 14 5 | 1 21
    24: 14 1
    
    abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
    bbabbbbaabaabba
    babbbbaabbbbbabbbbbbaabaaabaaa
    aaabbbbbbaaaabaababaabababbabaaabbababababaaa
    bbbbbbbaaaabbbbaaabbabaaa
    bbbababbbbaaaaaaaabbababaaababaabab
    ababaaaaaabaaab
    ababaaaaabbbaba
    baabbaaaabbaaaababbaababb
    abbbbabbbbaaaababbbbbbaaaababb
    aaaaabbaabaaaaababaa
    aaaabbaaaabbaaa
    aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
    babaaabbbaaabaababbaabababaaab
    aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"#;
    let lines = input.split("\n").map(|s| s.trim().to_owned()).collect();

    assert_eq!(solve(lines), 12);
  }
}
