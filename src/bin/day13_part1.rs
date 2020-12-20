
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

fn solve(lines: Vec<String>) -> i32 {
  // Get the timestamp
  let start_ts: i32 = lines[0].parse().unwrap();

  // Get the sequence of bus IDs
  let bus_ids: Vec<i32> = lines[1].split(",")
    .filter(|s| s != &"x")
    .map(|s| s.parse().unwrap())
    .collect();
  
  // Get the bus with the closest departure time
  let (bus_id, wait_time) = bus_ids.into_iter()
    .map(|id| (id, id - start_ts % id))
    .min_by_key(|(_, wait_time)| *wait_time)
    .unwrap();
  
  bus_id * wait_time
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
    assert_eq!(solve(lines), 295);
  }
}
