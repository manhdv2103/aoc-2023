use itertools::Itertools;
use regex::Regex;
use std::{
  char,
  cmp::{max, min},
  collections::{HashMap, HashSet},
  env, fs,
};

const PART: usize = 1;

fn solve_p1(input: String) -> String { return "".to_string(); }

fn solve_p2(input: String) -> String { return "".to_string(); }

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
