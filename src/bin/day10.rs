use aoc::prelude::*;
use itertools::Itertools;
use microlp::{ComparisonOp, OptimizationDirection, Problem};
use nom::bytes::complete::tag;
use nom::character::complete::{one_of, u64};
use nom::multi::{many1, separated_list1};
use nom::sequence::{delimited, separated_pair};

solution! {
  year: 2025,
  day: 10,
  parse,
  part_1,
  part_2,
}

type Parsed = Vec<(Vec<char>, Vec<Vec<u64>>, Vec<u64>)>;

fn parse(input: &str) -> Parsed {
  input
    .parse_lines(separated_pair(
      delimited(tag("["), many1(one_of(".#")), tag("]")),
      tag(" "),
      separated_pair(
        separated_list1(
          tag(" "),
          delimited(tag("("), separated_list1(tag(","), u64), tag(")")),
        ),
        tag(" "),
        delimited(tag("{"), separated_list1(tag(","), u64), tag("}")),
      ),
    ))
    .into_iter()
    .map(|(target, (buttons, joltages))| (target, buttons, joltages))
    .collect()
}

fn lights_to_u64(lights: &[char]) -> u64 {
  lights
    .iter()
    .fold(0u64, |acc, &c| (acc << 1) | (c == '#') as u64)
}

fn button_to_u64(button: &[u64], total_lights: u64) -> u64 {
  button
    .iter()
    .fold(0u64, |acc, &idx| acc | (1 << (total_lights - idx - 1)))
}

fn part_1(input: Parsed) -> u64 {
  let mut sum = 0;
  'machines: for (lights, buttons, _) in input {
    let target = lights_to_u64(&lights);
    let buttons: Vec<u64> = buttons
      .iter()
      .map(|b| button_to_u64(b, lights.len() as u64))
      .collect();

    for count in 1..=buttons.len() {
      for presses in buttons.iter().combinations(count) {
        let result = presses.iter().copied().fold(0u64, |acc, b| acc ^ b);
        if result == target {
          sum += count as u64;
          continue 'machines;
        }
      }
    }
  }
  sum
}

fn part_2(input: Parsed) -> u64 {
  let mut sum = 0;
  for (_, buttons, target_joltages) in input {
    let mut problem = Problem::new(OptimizationDirection::Minimize);
    let max_pushes = *target_joltages.iter().max().unwrap();
    let buttons = buttons
      .iter()
      .map(|button| (button, problem.add_integer_var(1.0, (0, max_pushes as i32))))
      .collect_vec();
    for (i, &target) in target_joltages.iter().enumerate() {
      let vars = buttons
        .iter()
        .filter(|(b, _)| b.contains(&(i as u64)))
        .map(|(_, var)| (*var, 1.0))
        .collect_vec();
      problem.add_constraint(&vars, ComparisonOp::Eq, target as f64);
    }
    sum += problem
      .solve()
      .expect("solution should exist")
      .objective()
      .round() as u64;
  }
  sum
}

#[cfg(test)]
mod tests {
  use super::*;
  use indoc::indoc;

  const SAMPLE_INPUT: &str = indoc! {"
    [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
    [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
    [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
  "};

  #[test]
  fn test_part1() {
    assert_eq!(part_1(parse(SAMPLE_INPUT)), 7);
  }

  #[test]
  fn test_part2() {
    assert_eq!(part_2(parse(SAMPLE_INPUT)), 33);
  }
}
