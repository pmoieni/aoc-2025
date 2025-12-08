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
        Self { name: "two" }
    }
}

impl Solution for Day1 {
    fn get_day(&self) -> &'static str {
        self.name
    }

    fn result_p1(&self) -> String {
        let file = fs::File::open("src/day2/input.txt").expect("failed to read input file");

        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line.expect("no line");
        }

        "".to_string()
    }

    fn result_p2(&self) -> String {
        let file = fs::File::open("src/day2/input.txt").expect("failed to read input file");

        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line.expect("no line");
        }

        "".to_string()
    }
}
