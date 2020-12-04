
use std::io;
use std::collections::HashSet;

fn main() {
  let mut input = String::new();
  let mut lines = vec![];

  loop {
    // Read inputs
    input.clear();
    let read_count = io::stdin().read_line(&mut input).unwrap();
    if read_count == 0 { // EOF
      break;
    }

    // Add to lines
    lines.push(input.trim().to_owned());
  }

  println!("{}", solve(&lines));
}

fn solve(lines: &Vec<String>) -> usize {
  let passports = parse_passports(lines);
  let mut count = 0;

  for passport in passports {
    if is_vaild(passport) {
      count += 1;
    }
  }

  count
}

fn parse_passports(lines: &Vec<String>) -> Vec<HashSet<&str>> {
  let mut passports = vec![];
  let mut passport = HashSet::new();

  for line in lines {
    if line.trim().is_empty() {
      passports.push(passport);
      passport = HashSet::new();
    } else {
      for token in line.trim().split(" ") {
        passport.insert(token.split(":").next().unwrap());
      }
    }
  }

  // The last passport
  passports.push(passport);

  passports
}

fn is_vaild(passport: HashSet<&str>) -> bool {
  let check_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

  for field in &check_fields {
    if !passport.contains(field) {
      return false;
    }
  }

  true
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test1() {
    let lines: Vec<String> = vec![
      "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd",
      "byr:1937 iyr:2017 cid:147 hgt:183cm",
      "",
      "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884",
      "hcl:#cfa07d byr:1929",
      "",
      "hcl:#ae17e1 iyr:2013",
      "eyr:2024",
      "ecl:brn pid:760753108 byr:1931",
      "hgt:179cm",
      "",
      "hcl:#cfa07d eyr:2025 pid:166559648",
      "iyr:2011 ecl:brn hgt:59in",
    ].into_iter().map(|s| s.to_owned()).collect();
    assert_eq!(solve(&lines), 2);
  }
}
