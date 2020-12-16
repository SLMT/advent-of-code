
use std::io;

#[derive(Debug, Clone, Copy)]
enum Direction {
  North,
  East,
  South,
  West
}

enum Rotation {
  Left,
  Right
}

#[derive(Debug)]
struct Ship {
  position: (i32, i32),
  waypoint: (i32, i32)
}

impl Ship {
  fn new() -> Ship {
    Ship {
      position: (0, 0),
      waypoint: (10, 1)
    }
  }

  fn move_waypoint(&mut self, direction: Direction, distance: i32) {
    self.waypoint = match direction {
      Direction::North => (self.waypoint.0, self.waypoint.1 + distance),
      Direction::East => (self.waypoint.0 + distance, self.waypoint.1),
      Direction::South => (self.waypoint.0, self.waypoint.1 - distance),
      Direction::West => (self.waypoint.0 - distance, self.waypoint.1),
    };
  }

  fn rotate_waypoint(&mut self, rotation: Rotation, degree: i32) {
    let times = degree / 90; // assume the degree must be multiples of 90

    for _ in 0 .. times {
      match rotation {
        Rotation::Left => self.rotate_waypoint_left_90(),
        Rotation::Right => self.rotate_waypoint_right_90()
      }
    }
  }

  fn rotate_waypoint_right_90(&mut self) {
    self.waypoint = (self.waypoint.1, -self.waypoint.0);
  }

  fn rotate_waypoint_left_90(&mut self) {
    self.waypoint = (-self.waypoint.1, self.waypoint.0);
  }

  fn forward(&mut self, times: i32) {
    for _ in 0 .. times {
      self.position = (
        self.position.0 + self.waypoint.0,
        self.position.1 + self.waypoint.1
      );
    }
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
      "N" => ship.move_waypoint(Direction::North, val),
      "S" => ship.move_waypoint(Direction::South, val),
      "E" => ship.move_waypoint(Direction::East, val),
      "W" => ship.move_waypoint(Direction::West, val),
      "L" => ship.rotate_waypoint(Rotation::Left, val),
      "R" => ship.rotate_waypoint(Rotation::Right, val),
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
    assert_eq!(solve(lines), 286);
  }
}
