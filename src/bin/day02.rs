use itertools::Itertools;
use regex::Regex;
use std::char;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;

const PART: usize = 1;

fn parse(input: String) -> Vec<(i32, Vec<Vec<(i32, String)>>)> {
    input
        .lines()
        .map(|game| {
            let (_, [game_id, game_info]) = Regex::new(r"Game (\d+): (.+)")
                .unwrap()
                .captures(game)
                .unwrap()
                .extract();

            (
                game_id.parse::<i32>().unwrap(),
                game_info
                    .split("; ")
                    .map(|cube_set| {
                        cube_set
                            .split(", ")
                            .map(|cube_type| {
                                let (cube_num, cube_color) =
                                    cube_type.splitn(2, ' ').collect_tuple().unwrap();
                                (cube_num.parse::<i32>().unwrap(), cube_color.to_string())
                            })
                            .collect()
                    })
                    .collect(),
            )
        })
        .collect()
}

fn solve_p1(input: String) -> String {
    let real_bag = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    parse(input)
        .iter()
        .map(|(game_id, game_info)| {
            let can_be_played = game_info.iter().all(|bag| {
                bag.iter().all(|(cube_num, cube_color)| {
                    cube_num <= real_bag.get(cube_color.as_str()).unwrap()
                })
            });

            if can_be_played {
                *game_id
            } else {
                0
            }
        })
        .sum::<i32>()
        .to_string()
}

fn solve_p2(input: String) -> String {
    return parse(input)
        .iter()
        .map(|(_, game_info)| {
            let max_cubes = game_info
                .iter()
                .fold(HashMap::new(), |mut max_cubes, cube_set| {
                    {
                        for (cube_num, cube_color) in cube_set {
                            max_cubes.insert(
                                cube_color,
                                *match max_cubes.get(cube_color) {
                                    Some(max_num) => max(max_num, &cube_num),
                                    None => &cube_num,
                                },
                            );
                        }
                    }

                    return max_cubes;
                });

            return max_cubes.values().fold(1, |power, num| power * *num);
        })
        .sum::<i32>()
        .to_string();
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
