/// 
/// The sample input is confusing. See https://www.reddit.com/r/adventofcode/comments/ker0wi/2020_day_17_part_1_sample_input_wrong/
/// 

use std::io;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

type Coordinate = (i32, i32, i32);
type Space = HashMap<Coordinate, Rc<RefCell<Cube>>>;

#[derive(PartialEq, Eq, Clone, Copy)]
enum CubeState {
  Active,
  Inactive
}

struct Cube {
  coordinate: Coordinate,
  neighbors: Option<Vec<Rc<RefCell<Cube>>>>,
  state: CubeState
}

impl Cube {
  fn new(coordinate: Coordinate, state: CubeState) -> Cube {
    Cube {
      coordinate,
      neighbors: None,
      state
    }
  }

  fn build_neighbors(&mut self, space: &mut Space) {
    if self.neighbors.is_some() {
      return;
    }

    let (x, y, z) = self.coordinate;
    let mut neighbors = vec![];

    for x_diff in -1..=1 {
      for y_diff in -1..=1 {
        for z_diff in -1..=1 {
          if x_diff != 0 || y_diff != 0 || z_diff != 0 {
            let near_coord = (x + x_diff, y + y_diff, z + z_diff);
            if let Some(neighbor) = space.get(&near_coord) {
              neighbors.push(neighbor.clone());
            } else {
              let neighbor = Rc::new(
                RefCell::new(Cube::new(near_coord, CubeState::Inactive))
              );
              space.insert(near_coord, neighbor.clone());
              neighbors.push(neighbor);
            }
          }
        }
      }
    }

    self.neighbors = Some(neighbors);
  }

  fn count_near_active(&self) -> i32 {
    if let Some(neighbors) = &self.neighbors {
      let mut active_count = 0;
      for neighbor in neighbors {
        if neighbor.borrow().state == CubeState::Active {
          active_count += 1;
        }
      }
      active_count
    } else {
      0
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

fn solve(lines: Vec<String>) -> i32 {
  let mut space = parse(lines);

  for _ in 0 .. 6 {
    expand(&mut space);
    update(&space);
  }
  
  count_active(&space)
}

fn parse(lines: Vec<String>) -> Space {
  let mut space = Space::new();
  let z = 0;
  let mut cubes = vec![];

  for (y, line) in lines.into_iter().enumerate() {
    for (x, ch) in line.chars().enumerate() {
      let state = match ch {
        '.' => CubeState::Inactive,
        '#' => CubeState::Active,
        _ => panic!("Unknown character: {}", ch)
      };
      let coordinate = (x as i32, y as i32, z);
      let cube = Rc::new(RefCell::new(Cube::new(coordinate, state)));
      cubes.push(cube.clone());
      space.insert(coordinate, cube);
    }
  }

  // Link neighbors
  for cube in cubes {
    cube.borrow_mut().build_neighbors(&mut space);
  }

  space
}

fn update(space: &Space) {
  let mut update_queue = vec![];

  // Check which cube needs to be updated
  for cube in space.values() {
    let active_count = cube.borrow().count_near_active();
    if active_count == 3 {
      update_queue.push((cube, CubeState::Active));
    } else if active_count != 2 {
      update_queue.push((cube, CubeState::Inactive));
    }
  }

  // To avoid interference
  for (cube, new_state) in update_queue {
    cube.borrow_mut().state = new_state;
  }
}

fn expand(space: &mut Space) {
  // Copy the cube list (to avoid RAII)
  let cubes: Vec<Rc<RefCell<Cube>>> = 
    space.values().map(|c| c.clone()).collect();

  for cube in cubes {
    cube.borrow_mut().build_neighbors(space);
  }
}

fn count_active(space: &Space) -> i32 {
  space.values().fold(0, |sum, cube| {
    if cube.borrow().state == CubeState::Active {
      sum + 1
    } else {
      sum
    }
  })
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test1() {
    let lines: Vec<String> = vec![
      ".#.",
      "..#",
      "###",
    ].into_iter().map(|s| s.to_owned()).collect();
    assert_eq!(solve(lines), 112);
  }
}
