use crate::aoc::{Day, Parser, Solution};

impl Solution<u8, u8> for Day<2024, 2> {
    fn part1(&self, input: &u8) -> u8 {
        *input
    }
}
impl Parser<u8> for Day<2024, 2> {
    fn parse(&self, input: String) -> anyhow::Result<u8> {
        Ok(input.trim().parse::<u8>()?)
    }
}
