
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

  // Count bags
  let mut count_cache = HashMap::new();
  count_bags(&contains_relations, &mut count_cache, "shiny gold")
}

fn parse_rule(line: &str) -> (&str, Vec<(usize, &str)>) {
  let mut inners = vec![];
  
  // Find outer bag color
  let mut tokens = line.split("bags contain");
  let outer = tokens.next().unwrap().trim();
  let tokens = tokens.next().unwrap().trim().split(",");
  for token in tokens {
    let inner = token.split("bag").next().unwrap().trim();
    if inner != "no other" {
      // Seperate the number and the color
      let space_index = inner.find(" ").unwrap();
      let number: usize = inner[.. space_index].parse().unwrap();
      let color = &inner[space_index + 1 ..];
      inners.push((number, color));
    }
  }

  (outer, inners)
}

// Note: needs to hint the compiler that
// all the strings are in the same lifetime
fn count_bags<'a>(
  relations: &HashMap<&'a str, Vec<(usize, &'a str)>>,
  count_cache: &mut HashMap<&'a str, usize>,
  outer: &'a str
) -> usize {
  if let Some(inners) = relations.get(outer) {
    let mut count = 0;

    for (inner_count, inner_color) in inners {
      if let Some(bag_count) = count_cache.get(inner_color) {
        count += (bag_count + 1) * inner_count;
      } else {
        let bag_count = count_bags(relations, count_cache, inner_color);
        count_cache.insert(*inner_color, bag_count);
        count += (bag_count + 1) * inner_count;
      }
    }

    count
  } else {
    0
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
    assert_eq!(solve(&lines), 32);
  }

  #[test]
  fn test2() {
    let lines: Vec<String> = vec![
      "shiny gold bags contain 2 dark red bags.",
      "dark red bags contain 2 dark orange bags.",
      "dark orange bags contain 2 dark yellow bags.",
      "dark yellow bags contain 2 dark green bags.",
      "dark green bags contain 2 dark blue bags.",
      "dark blue bags contain 2 dark violet bags.",
      "dark violet bags contain no other bags.",
    ].into_iter().map(|s| s.to_owned()).collect();
    assert_eq!(solve(&lines), 126);
  }
}
