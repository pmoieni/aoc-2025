use crate::day::Solution;
use std::{
    fs,
    io::{BufRead, BufReader},
};

pub struct Day1 {
    name: &'static str,
}

impl Day1 {
    pub fn new() -> Self {
        Self { name: "one" }
    }
}

impl Solution for Day1 {
    fn get_day(&self) -> &'static str {
        self.name
    }

    fn result_p1(&self) -> String {
        let file = fs::File::open("src/day1/input.txt").expect("failed to read input file");

        let reader = BufReader::new(file);

        let mut curr_idx = 50;
        let mut result = 0;

        for line in reader.lines() {
            let line = line.expect("no line");

            let (direction, distance) = line.split_at(1);
            let distance = distance.parse::<i16>().unwrap() % 100;
            match direction {
                "L" => {
                    println!("[BEFORE] L {} {}", distance, curr_idx);
                    curr_idx -= distance;
                    if curr_idx < 0 {
                        curr_idx += 100;
                    }
                    println!("[AFTER]  L {} {}\n", distance, curr_idx);
                }
                "R" => {
                    println!("[BEFORE] R {} {}", distance, curr_idx);
                    curr_idx += distance;
                    if curr_idx > 99 {
                        curr_idx -= 100;
                    }
                    println!("[AFTER]  R {} {}\n", distance, curr_idx);
                }
                _ => panic!("boom"),
            }

            if curr_idx == 0 {
                result += 1;
            }
        }

        result.to_string()
    }

    fn result_p2(&self) -> String {
        "".into()
    }
}
