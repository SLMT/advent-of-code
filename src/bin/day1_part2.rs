
use std::io;

fn main() {
  let mut input = String::new();
  let mut numbers = vec![];

  loop {
    // Read inputs
    input.clear();
    let read_count = io::stdin().read_line(&mut input).unwrap();
    if read_count == 0 { // EOF
      break;
    }

    // Parse the number
    let number: i32 = input.trim().parse().unwrap();
    numbers.push(number);
  }

  println!("{}", solve(&numbers));
}

fn solve(numbers: &Vec<i32>) -> i32 {
  for number_1 in numbers {
    for number_2 in numbers {
      // Find another number
      let number_3 = 2020 - number_1 - number_2;
  
      // Check existence
      if numbers.contains(&number_3) {
        return number_1 * number_2 * number_3;
      }
    }
  }

  panic!("there is no answer!");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test1() {
    let numbers = vec![1721, 979, 366, 299, 675, 1456];
    assert_eq!(solve(&numbers), 241861950);
  }
}
