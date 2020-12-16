
use std::io;

#[derive(Debug, Clone, Copy)]
enum Direction {
  North,
  East,
  South,
  West
}

impl Direction {
  fn right_next(&self) -> Direction {
    match self {
      Direction::North => Direction::East,
      Direction::East => Direction::South,
      Direction::South => Direction::West,
      Direction::West => Direction::North,
    }
  }

  fn left_next(&self) -> Direction {
    match self {
      Direction::North => Direction::West,
      Direction::East => Direction::North,
      Direction::South => Direction::East,
      Direction::West => Direction::South,
    }
  }
}

enum Rotation {
  Left,
  Right
}

struct Ship {
  facing: Direction,
  position: (i32, i32)
}

impl Ship {
  fn new() -> Ship {
    Ship {
      facing: Direction::East,
      position: (0, 0)
    }
  }

  fn move_with(&mut self, direction: Direction, distance: i32) {
    self.position = match direction {
      Direction::North => (self.position.0 + distance, self.position.1),
      Direction::East => (self.position.0, self.position.1 + distance),
      Direction::South => (self.position.0 - distance, self.position.1),
      Direction::West => (self.position.0, self.position.1 - distance),
    };
  }

  fn turn(&mut self, rotation: Rotation, degree: i32) {
    let times = degree / 90; // assume the degree must be multiples of 90

    for _ in 0 .. times {
      self.facing = match rotation {
        Rotation::Left => self.facing.left_next(),
        Rotation::Right => self.facing.right_next()
      }
    }
  }

  fn forward(&mut self, distance: i32) {
    self.move_with(self.facing, distance);
  }

  fn man_distance_from_start(&self) -> i32 {
    i32::abs(self.position.0) + i32::abs(self.position.1)
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

fn solve(lines: Vec<String>) -> i32 {
  let mut ship = Ship::new();

  for line in lines {
    let (ins, val) = split_instruction(&line);
    match ins {
      "N" => ship.move_with(Direction::North, val),
      "S" => ship.move_with(Direction::South, val),
      "E" => ship.move_with(Direction::East, val),
      "W" => ship.move_with(Direction::West, val),
      "L" => ship.turn(Rotation::Left, val),
      "R" => ship.turn(Rotation::Right, val),
      "F" => ship.forward(val),
      _ => panic!("Unknown instruction: {}", ins)
    }
  }

  ship.man_distance_from_start()
}

fn split_instruction(line: &str) -> (&str, i32) {
  (&line[0..1], line[1..].parse().unwrap())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test1() {
    let lines: Vec<String> = vec![
      "F10",
      "N3",
      "F7",
      "R90",
      "F11",
    ].into_iter().map(|s| s.to_owned()).collect();
    assert_eq!(solve(lines), 25);
  }
}
