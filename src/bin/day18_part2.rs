
use std::io;

enum Op {
  Add,
  Mul
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
  let mut sum = 0;

  for line in lines {
    sum += eval_line(&line);
  }

  sum
}

fn eval_line(line: &str) -> i64 {
  // Add more "(" and ")" to highlight the pority of '+'
  // Make "A + B * (C * D + E)" become
  // "( A + B ) * ( ( ( C ) * ( D + E ) )"
  let new_line = line
    .replace("(", " ( ( ")
    .replace(")", " ) ) ")
    .replace("*", ") * (");
  let new_line = format!("( {} )", new_line);
  let mut tokens = new_line.split(" ")
    .filter(|s| !s.is_empty());

  eval(&mut tokens)
}

fn eval<'a, I>(tokens: &mut I) -> i64
  where I: Iterator<Item = &'a str> {
  
  let mut result = 0;
  let mut last_op = Op::Add;

  while let Some(token) = tokens.next() {
    match token {
      "+" => last_op = Op::Add,
      "*" => last_op = Op::Mul,
      "(" => {
        let inner_result = eval(tokens);
        match last_op {
          Op::Add => result += inner_result,
          Op::Mul => result *= inner_result
        }
      },
      ")" => return result,
      num_str => { // Assume this is a number
        let number: i64 = num_str.parse().unwrap();
        match last_op {
          Op::Add => result += number,
          Op::Mul => result *= number
        }
      }
    }
  }
  
  result
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test1() {
    let lines: Vec<String> = vec![
      "1 + 2 * 3 + 4 * 5 + 6",
    ].into_iter().map(|s| s.to_owned()).collect();
    assert_eq!(solve(lines), 231);
  }

  #[test]
  fn test2() {
    let lines: Vec<String> = vec![
      "1 + (2 * 3) + (4 * (5 + 6))",
    ].into_iter().map(|s| s.to_owned()).collect();
    assert_eq!(solve(lines), 51);
  }

  #[test]
  fn test3() {
    let lines: Vec<String> = vec![
      "2 * 3 + (4 * 5)",
    ].into_iter().map(|s| s.to_owned()).collect();
    assert_eq!(solve(lines), 46);
  }

  #[test]
  fn test4() {
    let lines: Vec<String> = vec![
      "5 + (8 * 3 + 9 + 3 * 4 * 3)",
    ].into_iter().map(|s| s.to_owned()).collect();
    assert_eq!(solve(lines), 1445);
  }

  #[test]
  fn test5() {
    let lines: Vec<String> = vec![
      "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))",
    ].into_iter().map(|s| s.to_owned()).collect();
    assert_eq!(solve(lines), 669060);
  }

  #[test]
  fn test6() {
    let lines: Vec<String> = vec![
      "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2",
    ].into_iter().map(|s| s.to_owned()).collect();
    assert_eq!(solve(lines), 23340);
  }
}
