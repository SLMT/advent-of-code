
use std::io;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Seat {
  Floor,
  Empty,
  Occupied
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

fn solve(lines: Vec<String>) -> usize {
  let mut seats = into_seats(lines);

  loop {
    let (new_seats, more_to_go) = occupy_or_abandon(seats);
    seats = new_seats;
    if !more_to_go {
      break;
    }
  }

  seats
  .iter()
  .fold(0,
    |count, row|
    count + row.iter().filter(|s| **s == Seat::Occupied).count()
  )
}

fn into_seats(lines: Vec<String>) -> Vec<Vec<Seat>> {
  let mut seats = vec![];

  for line in lines {
    let mut row = vec![];
    for ch in line.chars() {
      let seat = match ch {
        '.' => Seat::Floor,
        'L' => Seat::Empty,
        '#' => Seat::Occupied,
        _ => panic!("Unknown character: {}", ch)
      };
      row.push(seat);
    }
    seats.push(row);
  }

  seats
}

fn occupy_or_abandon(seats: Vec<Vec<Seat>>) -> (Vec<Vec<Seat>>, bool) {
  let mut new_seats = vec![];
  let mut has_changed = false;

  for (y, row) in seats.iter().enumerate() {
    let mut new_row = vec![];
    for (x, seat) in row.iter().enumerate() {
      if let Some(new_seat) = match *seat {
        Seat::Empty => {
          if count_adjacent_occupied(&seats, x, y) == 0 {
            Some(Seat::Occupied)
          } else {
            None
          }
        },
        Seat::Occupied => {
          if count_adjacent_occupied(&seats, x, y) >= 4 {
            Some(Seat::Empty)
          } else {
            None
          }
        },
        _ => None
      } {
        new_row.push(new_seat);
        has_changed = true;
      } else {
        new_row.push(*seat);
      }
    }
    new_seats.push(new_row);
  }

  (new_seats, has_changed)
}

fn count_adjacent_occupied(
  seats: &Vec<Vec<Seat>>,
  x: usize, y: usize,
) -> usize {
  let start_x = if x > 0 { x - 1 } else { 0 };
  let start_y = if y > 0 { y - 1 } else { 0 };
  let end_x = std::cmp::min(seats[0].len() - 1, x + 1) + 1;
  let end_y = std::cmp::min(seats.len() - 1, y + 1) + 1;

  let mut count = 0;

  for cx in start_x .. end_x {
    for cy in start_y .. end_y {
      if cx == x && cy == y {
        continue;
      } else if seats[cx][cy] == Seat::Occupied {
        count += 1;
      }
    }
  }

  count
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_all() {
    let lines: Vec<String> = vec![
      "L.LL.LL.LL",
      "LLLLLLL.LL",
      "L.L.L..L..",
      "LLLL.LL.LL",
      "L.LL.LL.LL",
      "L.LLLLL.LL",
      "..L.L.....",
      "LLLLLLLLLL",
      "L.LLLLLL.L",
      "L.LLLLL.LL",
    ].into_iter().map(|s| s.to_owned()).collect();
    assert_eq!(solve(lines), 37);
  }
}
