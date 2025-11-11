use crate::aoc::{Day, Solution};

impl Solution for Day<2024, 1> {
    fn part1(&self, input: &String) -> String {
        input.lines().count().to_string()
    }
}
