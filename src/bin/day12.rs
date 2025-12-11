use aoc::prelude::*;
use itertools::Itertools;
use std::collections::HashMap;

solution! {
  year: 2025,
  day: 12,
  parse,
  part_1,
  part_2: |_| 0,
}

type Parsed = (Vec<(u64, Vec<Vec<u8>>)>, Vec<((u64, u64), Vec<u64>)>);

fn parse(input: &str) -> Parsed {
  use parse::*;

  input.parse_full(separated_pair(
    separated_list1(
      tag("\n\n"),
      separated_pair(
        u64,
        tag(":\n"),
        separated_list1(tag("\n"), many1(one_of("#.").map(|c| (c == '#') as u8))),
      ),
    ),
    tag("\n\n"),
    separated_list1(
      tag("\n"),
      separated_pair(
        separated_pair(u64, tag("x"), u64),
        tag(": "),
        separated_list1(tag(" "), u64),
      ),
    ),
  ))
}

fn part_1((shapes, regions): Parsed) -> u64 {
  let shapes = HashMap::<_, _>::from_iter(shapes);
  let sizes = shapes
    .values()
    .map(|v| v.iter().flat_map(|r| r.iter()).sum::<u8>())
    .collect_vec();

  let mut sum = 0;
  for ((w, h), counts) in regions {
    let total = w * h;
    let req = counts
      .iter()
      .enumerate()
      .map(|(i, &c)| c * sizes[i] as u64)
      .sum::<u64>();

    if (req as f64 / total as f64) < 0.8 {
      sum += 1;
    }
  }
  sum
}
