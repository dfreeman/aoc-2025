use std::str::Lines;

use aoc::prelude::*;
use nom::{
  character::complete::{newline, one_of, space0, space1, u64},
  multi::separated_list1,
  sequence::{delimited, separated_pair},
};

solution! {
  year: 2025,
  day: 6,
  parse,
  part_1,
  part_2,
}

enum Op {
  Add,
  Mul,
}

impl Op {
  fn fold(&self, nums: &[u64]) -> u64 {
    nums
      .iter()
      .fold(self.identity(), |acc, &num| self.apply(acc, num))
  }

  fn apply(&self, a: u64, b: u64) -> u64 {
    match self {
      Op::Add => a + b,
      Op::Mul => a * b,
    }
  }

  fn identity(&self) -> u64 {
    match self {
      Op::Add => 0,
      Op::Mul => 1,
    }
  }

  fn parse(c: char) -> Self {
    match c {
      '+' => Op::Add,
      '*' => Op::Mul,
      _ => panic!("Invalid operator character: {}", c),
    }
  }
}

fn part_1(input: &String) -> u64 {
  let (rows, ops) = input.parse_full(separated_pair(
    separated_list1(
      newline,
      delimited(space0, separated_list1(space1, u64), space0),
    ),
    newline,
    delimited(space0, separated_list1(space1, one_of("+*")), space0),
  ));
  let ops = ops.into_iter().map(Op::parse);
  let rows = rows.transpose().into_iter();

  let mut sum = 0;
  for (op, nums) in ops.zip(rows) {
    sum += op.fold(&nums);
  }
  sum
}

fn collect_vertical_problems(lines: Lines) -> Vec<Vec<u64>> {
  let cols = lines
    .map(|line| line.chars().collect())
    .collect::<Vec<Vec<char>>>()
    .transpose();

  let mut problems = Vec::new();
  let mut current_problem = Vec::new();
  for col in cols {
    let value = col.into_iter().collect::<String>().trim().to_string();
    if value == "" {
      problems.push(current_problem);
      current_problem = Vec::new();
    } else {
      current_problem.push(value.parse::<u64>().unwrap());
    }
  }
  problems.push(current_problem);
  problems
}

fn part_2(input: &String) -> u64 {
  let mut lines = input.trim().lines();
  let ops = lines
    .next_back()
    .unwrap()
    .parse_full(separated_list1(space1, one_of("+*")))
    .into_iter()
    .map(Op::parse);
  let problems = collect_vertical_problems(lines).into_iter();

  let mut sum = 0;
  for (op, nums) in ops.zip(problems) {
    sum += op.fold(&nums);
  }
  sum
}

#[cfg(test)]
mod tests {
  use super::*;
  use indoc::indoc;

  const SAMPLE_INPUT: &str = indoc! {"
    123 328  51 64 
      45 64  387 23 
      6 98  215 314
    *   +   *   +  
  "};

  #[test]
  fn test_part1() {
    assert_eq!(part_1(&parse(SAMPLE_INPUT)), 4277556);
  }

  #[test]
  fn test_part2() {
    assert_eq!(part_2(&parse(SAMPLE_INPUT)), 3263827);
  }
}
