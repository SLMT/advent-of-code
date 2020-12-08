
use std::io;
use std::collections::HashMap;

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
  let mut contains_relations = HashMap::new();

  // Parse rules
  for line in lines {
    let (outer, inners) = parse_rule(line);
    contains_relations.insert(outer, inners);
  }

  // Check contains relationships
  contains_relations
    .keys()
    .filter(|outer| has_shiny_gold(&contains_relations, outer))
    .count()
}

fn parse_rule(line: &str) -> (&str, Vec<&str>) {
  let mut inners = vec![];
  
  // Find outer bag color
  let mut tokens = line.split("bags contain");
  let outer = tokens.next().unwrap().trim();
  let tokens = tokens.next().unwrap().trim().split(",");
  for token in tokens {
    let inner = token.split("bag").next().unwrap().trim();
    if inner != "no other" {
      // Remove the number
      let start_idx = inner.find(" ").unwrap() + 1;
      inners.push(&inner[start_idx ..]);
    }
  }

  (outer, inners)
}

fn has_shiny_gold(relations: &HashMap<&str, Vec<&str>>, outer: &str) -> bool {
  if let Some(inners) = relations.get(outer) {
    for inner in inners {
      if *inner == "shiny gold" {
        return true;
      } else if has_shiny_gold(relations, *inner) {
        return true;
      }
    }

    false
  } else {
    false
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test1() {
    let lines: Vec<String> = vec![
      "light red bags contain 1 bright white bag, 2 muted yellow bags.",
      "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
      "bright white bags contain 1 shiny gold bag.",
      "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
      "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
      "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
      "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
      "faded blue bags contain no other bags.",
      "dotted black bags contain no other bags.",
    ].into_iter().map(|s| s.to_owned()).collect();
    assert_eq!(solve(&lines), 4);
  }
}
