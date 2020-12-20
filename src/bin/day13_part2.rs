
use std::io;

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
  // Get the sequence of bus IDs
  let bus_ids: Vec<Option<i64>> = lines[1].split(",")
    .map(|s| s.parse().ok())
    .collect();
  
  // Convert to a Chinese remainder problem
  let cr_sequence: Vec<(i64, i64)> = bus_ids.into_iter()
    .zip((0..).into_iter())
    .filter(|(id, _)| id.is_some())
    .map(|(id, wait_time)| (id.unwrap(), wait_time))
    .map(|(id, wait_time)| (id, (id - (wait_time % id)) % id))
    .collect();

  solve_chinese_remainder(cr_sequence)
}

// Chinese remainder problem
// Input: a sequence of (modular number, remainder) pair
fn solve_chinese_remainder(problem: Vec<(i64, i64)>) -> i64 {
  // Take out the first number
  let mut solution = problem[0].1;
  let mut least_common_multiple = problem[0].0;

  for (m, a) in problem.into_iter().skip(1) {
    // We want to find a solution x such that
    // x % m = a
    while solution % m != a {
      solution += least_common_multiple;
    }

    least_common_multiple *= m;
  }

  solution
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test1() {
    let lines: Vec<String> = vec![
      "939",
      "7,13,x,x,59,x,31,19",
    ].into_iter().map(|s| s.to_owned()).collect();
    assert_eq!(solve(lines), 1068781);
  }

  #[test]
  fn test2() {
    let lines: Vec<String> = vec![
      "939",
      "17,x,13,19",
    ].into_iter().map(|s| s.to_owned()).collect();
    assert_eq!(solve(lines), 3417);
  }

  #[test]
  fn test3() {
    let lines: Vec<String> = vec![
      "939",
      "67,7,59,61",
    ].into_iter().map(|s| s.to_owned()).collect();
    assert_eq!(solve(lines), 754018);
  }

  #[test]
  fn test4() {
    let lines: Vec<String> = vec![
      "939",
      "67,x,7,59,61",
    ].into_iter().map(|s| s.to_owned()).collect();
    assert_eq!(solve(lines), 779210);
  }

  #[test]
  fn test5() {
    let lines: Vec<String> = vec![
      "939",
      "67,7,x,59,61",
    ].into_iter().map(|s| s.to_owned()).collect();
    assert_eq!(solve(lines), 1261476);
  }

  #[test]
  fn test6() {
    let lines: Vec<String> = vec![
      "939",
      "1789,37,47,1889",
    ].into_iter().map(|s| s.to_owned()).collect();
    assert_eq!(solve(lines), 1202161486);
  }
}
