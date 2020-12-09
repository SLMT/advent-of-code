
use std::io;

#[derive(Debug)]
enum Instruction {
  Nop,
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

  execute_until_loop(&instructions)
}

fn parse_instruction(line: &str) -> Instruction {
  let mut tokens = line.split(" ");
  match tokens.next().unwrap() {
    "nop" => Instruction::Nop,
    "acc" => Instruction::Acc(tokens.next().unwrap().parse().unwrap()),
    "jmp" => Instruction::Jmp(tokens.next().unwrap().parse().unwrap()),
    unknown => panic!("Unknown instruction: {}", unknown)
  }
}

fn execute_until_loop(instructions: &Vec<Instruction>) -> i32 {
  let mut pc = 0; // Program counter
  let mut acc = 0; // Accumulator
  let mut used = vec![false; instructions.len()]; // Loop detector

  while !used[pc] {
    used[pc] = true;
    pc = match instructions[pc] {
      Instruction::Nop => pc + 1,
      Instruction::Acc(val) => {
        acc += val;
        pc + 1
      },
      Instruction::Jmp(diff) => (pc as i32 + diff) as usize
    };
  }

  acc
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
    assert_eq!(solve(&lines), 5);
  }
}
