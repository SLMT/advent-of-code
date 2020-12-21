
use std::io;

#[derive(Debug, Clone)]
struct MaskedBits {
  bits: Vec<Option<bool>>
}

impl MaskedBits {
  fn from_str(string: &str) -> MaskedBits {
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

  fn new() -> MaskedBits {
    MaskedBits {
      bits: vec![None; 36]
    }
  }

  fn mask(&self, input: i64) -> MaskedBits {
    // Change the input to binary string
    let old_bits = MaskedBits::from_str(&format!("{:036b}", input));

    // Mask the bits
    let mut new_bits = vec![None; 36];
    for (old_bit, mask) in old_bits.bits.iter().zip(self.bits.iter()) {
      match mask {
        &Some(true) => new_bits.push(Some(true)),
        &Some(false) => new_bits.push(old_bit.clone()),
        &None => new_bits.push(None),
      }
    }

    // Output as a MaskedBits object
    MaskedBits {
      bits: new_bits
    }
  }
}

#[derive(Debug)]
enum Instruction {
  SetMask(MaskedBits),
  SetValue(usize, i64) // (address, value)
}

impl Instruction {
  fn from_str(line: &str) -> Instruction {
    match &line[0..3] {
      "mas" => {
        let mask = line.split("=").nth(1).unwrap().trim();
        Instruction::SetMask(MaskedBits::from_str(mask))
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

struct MemoryData {
  address: Option<MaskedBits>,
  data: i64
}

impl MemoryData {
  fn new(address: MaskedBits, data: i64) -> MemoryData {
    MemoryData {
      address: Some(address),
      data
    }
  }

  fn earse(&mut self, address: &MaskedBits) {
    todo!();
  }

  fn sum(&self) -> i64 {
    todo!()
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

  let mut memory = vec![];
  let mut mask = MaskedBits::new();
  for ins in instructions {
    execute(&ins, &mut mask, &mut memory);
  }

  memory.iter().map(|data| data.sum()).fold(0, |sum, val| sum + val)
}

fn execute(
  ins: &Instruction,
  mask: &mut MaskedBits,
  memory: &mut Vec<MemoryData>
) {
  match ins {
    Instruction::SetMask(new_mask) => *mask = new_mask.clone(),
    Instruction::SetValue(addr, value) => {
      let address = mask.mask(*addr as i64);

      // Erase the overlapped data
      for data in memory.iter_mut() {
        data.earse(&address);
      }

      // Insert the new data
      memory.push(MemoryData::new(address, *value));
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test1() {
    let lines: Vec<String> = vec![
      "mask = 000000000000000000000000000000X1001X",
      "mem[42] = 100",
      "mask = 00000000000000000000000000000000X0XX",
      "mem[26] = 1",
    ].into_iter().map(|s| s.to_owned()).collect();
    assert_eq!(solve(lines), 208);
  }
}
