
use std::io;
use std::collections::HashSet;

fn main() {
  let mut input = String::new();
  let mut lines = vec![];

  // Read inputs
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

fn solve(lines: &Vec<String>) -> usize {
  let passports = parse_passports(lines);
  let mut count = 0;

  for passport in passports {
    if is_vaild_passport(passport) {
      count += 1;
    }
  }

  count
}

fn parse_passports(lines: &Vec<String>) -> Vec<HashSet<&str>> {
  let mut passports = vec![];
  let mut valid_fields = HashSet::new();

  for line in lines {
    if line.trim().is_empty() {
      passports.push(valid_fields);
      valid_fields = HashSet::new();
    } else {
      for token in line.trim().split(" ") {
        let mut tokens = token.split(":");
        let key = tokens.next().unwrap().trim();
        let value = tokens.next().unwrap().trim();
        match key {
          "byr" => {
            if is_valid_year(value, 1920, 2002) {
              valid_fields.insert(key);
            }
          },
          "iyr" => {
            if is_valid_year(value, 2010, 2020) {
              valid_fields.insert(key);
            }
          },
          "eyr" => {
            if is_valid_year(value, 2020, 2030) {
              valid_fields.insert(key);
            }
          },
          "hgt" => {
            if is_valid_height(value, "cm", 150, 193) {
              valid_fields.insert(key);
            } else if is_valid_height(value, "in", 59, 76) {
              valid_fields.insert(key);
            }
          },
          "hcl" => {
            if value.split("#").count() == 2 {
              let number = value.split("#").nth(1).unwrap();
              if number.len() == 6 {
                if i32::from_str_radix(number, 10).is_ok() ||
                    i32::from_str_radix(number, 16).is_ok() {
                  valid_fields.insert(key);
                }
              }
            }
          },
          "ecl" => {
            if ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value) {
              valid_fields.insert(key);
            }
          },
          "pid" => {
            if value.len() == 9 {
              if value.parse::<i64>().is_ok() {
                valid_fields.insert(key);
              }
            }
          },
          _ => {}
        }
      }
    }
  }

  // The last passport
  passports.push(valid_fields);

  passports
}

fn is_vaild_passport(passport: HashSet<&str>) -> bool {
  passport.len() == 7
}

fn is_valid_year(value: &str, start: i32, end: i32) -> bool {
  if value.len() == 4 {
    let year: i32 = value.parse().unwrap();
    if year >= start && year <= end {
      return true;
    }
  }

  false
}

fn is_valid_height(value: &str, metric: &str, start: i32, end: i32) -> bool {
  if value.split(metric).count() == 2 {
    let number = value.split(metric).next().unwrap();
    if let Ok(height) = number.parse::<i32>() {
      if height >= start && height <= end {
        return true;
      }
    }
  }

  false
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test1() {
    let lines: Vec<String> = vec![
      "eyr:1972 cid:100",
      "hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926",
      "",
      "iyr:2019",
      "hcl:#602927 eyr:1967 hgt:170cm",
      "ecl:grn pid:012533040 byr:1946",
      "",
      "hcl:dab227 iyr:2012",
      "ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277",
      "",
      "hgt:59cm ecl:zzz",
      "eyr:2038 hcl:74454a iyr:2023",
      "pid:3556412378 byr:2007",
    ].into_iter().map(|s| s.to_owned()).collect();
    assert_eq!(solve(&lines), 0);
  }

  #[test]
  fn test2() {
    let lines: Vec<String> = vec![
      "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980",
      "hcl:#623a2f",
      "",
      "eyr:2029 ecl:blu cid:129 byr:1989",
      "iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm",
      "",
      "hcl:#888785",
      "hgt:164cm byr:2001 iyr:2015 cid:88",
      "pid:545766238 ecl:hzl",
      "eyr:2022",
      "",
      "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719",
      "",
      "",
    ].into_iter().map(|s| s.to_owned()).collect();
    assert_eq!(solve(&lines), 4);
  }
}
