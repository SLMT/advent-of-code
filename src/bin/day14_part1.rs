
use std::io;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Mask {
  bits: Vec<Option<bool>>
}

impl Mask {
  fn from_str(string: &str) -> Mask {
    let mut mask = Self::new();

    for (idx, ch) in string.chars().enumerate() {
      mask.bits[idx] = match ch {
        '0' => Some(false),
        '1' => Some(true),
        'X' => None,
        _ => panic!("Unknown character: {}", ch)
      };
    }

    mask
  }

  fn new() -> Mask {
    Mask {
      bits: vec![None; 36]
    }
  }

  fn mask(&self, input: i64) -> i64 {
    // Change the input to binary string
    let binary = format!("{:036b}", input);

    // Mask the bits
    let mut new_binary = String::new();
    for (old_bit, mask) in binary.chars().zip(self.bits.iter()) {
      match mask {
        &Some(true) => new_binary.push('1'),
        &Some(false) => new_binary.push('0'),
        &None => new_binary.push(old_bit),
      }
    }

    // Convert back to a number
    i64::from_str_radix(&new_binary, 2).unwrap()
  }
}

#[derive(Debug)]
enum Instruction {
  SetMask(Mask),
  SetValue(usize, i64) // (address, value)
}

impl Instruction {
  fn from_str(line: &str) -> Instruction {
    match &line[0..3] {
      "mas" => {
        let mask = line.split("=").nth(1).unwrap().trim();
        Instruction::SetMask(Mask::from_str(mask))
      },
      "mem" => {
        let mut tokens = line.split("=");
        let address: usize = tokens.next().unwrap()
          .split("[").nth(1).unwrap()
          .split("]").next().unwrap()
          .trim().parse().unwrap();
        let value: i64 = tokens.next().unwrap()
          .trim().parse().unwrap();
        Instruction::SetValue(address, value)
      },
      _ => panic!("Unknown instruction: {}", line)
    }
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
  let instructions: Vec<Instruction> = lines.into_iter()
    .map(|line| Instruction::from_str(&line))
    .collect();

  let mut memory: HashMap<usize, i64> = HashMap::new();
  let mut mask = Mask::new();
  for ins in instructions {
    execute(&ins, &mut mask, &mut memory);
  }

  memory.values().fold(0, |sum, value| sum + value)
}

fn execute(
  ins: &Instruction,
  mask: &mut Mask,
  memory: &mut HashMap<usize, i64>
) {
  match ins {
    Instruction::SetMask(new_mask) => *mask = new_mask.clone(),
    Instruction::SetValue(addr, value) => {
      memory.insert(*addr, mask.mask(*value));
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test1() {
    let lines: Vec<String> = vec![
      "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X",
      "mem[8] = 11",
      "mem[7] = 101",
      "mem[8] = 0",
    ].into_iter().map(|s| s.to_owned()).collect();
    assert_eq!(solve(lines), 165);
  }
}
