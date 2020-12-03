
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
  let map = parse_map(lines);

  let (mut x, mut y) = (0, 0);
  let mut tree_count = 0;
  while y < map.len() {
    if !map[y][x] {
      tree_count += 1;
    }
    
    // advance
    x = (x + 3) % map[0].len();
    y += 1;
  }

  tree_count
}

fn parse_map(lines: &Vec<String>) -> Vec<Vec<bool>> {
  let mut map = vec![];

  for line in lines {
    let mut row = vec![];
    for ch in line.chars() {
      match ch {
        '.' => row.push(true),
        '#' => row.push(false),
        _ => panic!("Unknown charactor: {}", ch)
      }
    }
    map.push(row);
  }

  map
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test1() {
    let lines: Vec<String> = vec![
      "..##.......",
      "#...#...#..",
      ".#....#..#.",
      "..#.#...#.#",
      ".#...##..#.",
      "..#.##.....",
      ".#.#.#....#",
      ".#........#",
      "#.##...#...",
      "#...##....#",
      ".#..#...#.#",
    ].into_iter().map(|s| s.to_owned()).collect();
    assert_eq!(solve(&lines), 7);
  }
}
