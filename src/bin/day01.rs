use regex::Regex;
use std::{
  char,
  collections::{HashMap, HashSet},
  env, fs,
};

const PART: usize = 1;

fn solve(input: String, digit_values: &[(&str, i32)]) -> i32 {
  return input
    .lines()
    .map(|line| {
      let ((_, first_digit), (_, last_digit)) = digit_values
        .iter()
        .flat_map(|(digit, value)| {
          line.match_indices(digit).map(|(index, _)| (index as i32, *value)).into_iter()
        })
        .fold(((-1, -1), (-1, -1)), |(min, max), item| {
          if min.0 == -1 {
            (item, item)
          } else {
            (if item.0 < min.0 { item } else { min }, if item.0 > max.0 { item } else { max })
          }
        });

      return first_digit * 10 + last_digit;
    })
    .sum();
}

fn solve_p1(input: String) -> String {
  return solve(
    input,
    &[("1", 1), ("2", 2), ("3", 3), ("4", 4), ("5", 5), ("6", 6), ("7", 7), ("8", 8), ("9", 9)],
  )
  .to_string();
}

fn solve_p2(input: String) -> String {
  return solve(
    input,
    &[
      ("one", 1),
      ("two", 2),
      ("three", 3),
      ("four", 4),
      ("five", 5),
      ("six", 6),
      ("seven", 7),
      ("eight", 8),
      ("nine", 9),
      ("1", 1),
      ("2", 2),
      ("3", 3),
      ("4", 4),
      ("5", 5),
      ("6", 6),
      ("7", 7),
      ("8", 8),
      ("9", 9),
    ],
  )
  .to_string();
}

fn main() {
  if let Some(filename) = env::args().nth(0) {
    let day =
      Regex::new(r"\d+$").unwrap().find(&filename).unwrap().as_str().parse::<i32>().unwrap();
    println!("Day {}", day);

    let mut input_file: String = "./input/day".to_owned();
    input_file.push_str(format!("{:0>2}", day).as_str());
    let input = fs::read_to_string(input_file).unwrap();

    println!("Part {}:", PART);
    let result = (if PART == 1 { solve_p1 } else { solve_p2 })(input);
    println!("{}", result);
  }
}
