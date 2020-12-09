
use std::io;

#[derive(Debug, Clone, Copy)]
enum Instruction {
  Nop(i32),
  Acc(i32),
  Jmp(i32)
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

  println!("{}", solve(&lines));
}

fn solve(lines: &Vec<String>) -> i32 {
  // Parse instructions
  let mut instructions = vec![];
  for line in lines {
    instructions.push(parse_instruction(line));
  }

  // Find the wrong instruction
  for (idx, ins) in instructions.iter().enumerate() {
    if let Some(result) = match ins {
      Instruction::Nop(val) =>
        execute(&instructions, (idx, Instruction::Jmp(*val))),
      Instruction::Jmp(val) =>
        execute(&instructions, (idx, Instruction::Nop(*val))),
      _ => None
    } {
      return result;
    }
  }

  panic!("cannot fix the program");
}

fn parse_instruction(line: &str) -> Instruction {
  let mut tokens = line.split(" ");
  let op = tokens.next().unwrap();
  let val = tokens.next().unwrap().parse().unwrap();
  match op {
    "nop" => Instruction::Nop(val),
    "acc" => Instruction::Acc(val),
    "jmp" => Instruction::Jmp(val),
    unknown => panic!("Unknown instruction: {}", unknown)
  }
}

fn execute(
  instructions: &Vec<Instruction>,
  overwrite: (usize, Instruction)
) -> Option<i32> {
  let mut pc = 0; // Program counter
  let mut acc = 0; // Accumulator
  let mut used = vec![false; instructions.len()]; // Loop detector
  let (ow_idx, ow_ins) = overwrite;

  while pc < instructions.len() {
    // Loop detection
    if used[pc] {
      return None;
    }
    used[pc] = true;

    // Overwirtes?
    let ins = if ow_idx == pc {
      ow_ins
    } else {
      instructions[pc]
    };

    // Execute the instruction
    pc = match ins {
      Instruction::Nop(_) => pc + 1,
      Instruction::Acc(val) => {
        acc += val;
        pc + 1
      },
      Instruction::Jmp(diff) => (pc as i32 + diff) as usize
    };
  }

  Some(acc)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test1() {
    let lines: Vec<String> = vec![
      "nop +0",
      "acc +1",
      "jmp +4",
      "acc +3",
      "jmp -3",
      "acc -99",
      "acc +1",
      "jmp -4",
      "acc +6",
    ].into_iter().map(|s| s.to_owned()).collect();
    assert_eq!(solve(&lines), 8);
  }
}
