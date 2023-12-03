use regex::Regex;
use std::char;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;

const PART: u32 = 1;

fn solve_p1(input: String) -> String {
    return "".to_string();
}

fn solve_p2(input: String) -> String {
    return "".to_string();
}

fn main() {
    if let Some(filename) = env::args().nth(0) {
        let day = Regex::new(r"\d+$")
            .unwrap()
            .find(&filename)
            .unwrap()
            .as_str()
            .parse::<i32>()
            .unwrap();

        println!("Day {}", day);

        let mut input_file: String = "./input/day".to_owned();
        input_file.push_str(format!("{:0>2}", day).as_str());

        let input = fs::read_to_string(input_file).unwrap();

        println!("Part {}:", PART);
        let result = (if PART == 1 { solve_p1 } else { solve_p2 })(input);
        println!("{}", result);
    }
}
