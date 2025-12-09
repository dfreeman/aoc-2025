use aoc::prelude::*;
use interval::prelude::*;
use nom::{bytes::complete::tag, character::complete::u64, sequence::separated_pair};
use std::collections::BTreeMap;

solution! {
  year: 2025,
  day: 9,
  parse,
  part_1,
  part_2,
}

fn parse(input: &str) -> Vec<(u64, u64)> {
  input.parse_lines(separated_pair(u64, tag(","), u64))
}

fn part_1(points: &Vec<(u64, u64)>) -> u64 {
  let mut max = 0;
  for (i, &(x1, y1)) in points.iter().enumerate() {
    for &(x2, y2) in points.iter().skip(i + 1) {
      let area = (x1.abs_diff(x2) + 1) * (y1.abs_diff(y2) + 1);
      if area > max {
        max = area;
      }
    }
  }
  max
}

fn sort(a: u64, b: u64) -> (u64, u64) {
  if a <= b { (a, b) } else { (b, a) }
}

fn sort_and_inset(a: u64, b: u64) -> (u64, u64) {
  let (a, b) = sort(a, b);
  (a + 1, b - 1)
}

fn add_border_segment(
  border_segments: &mut BTreeMap<u64, IntervalSet<u64>>,
  coord: u64,
  start: u64,
  end: u64,
) {
  border_segments
    .entry(coord)
    .or_insert_with(|| IntervalSet::empty())
    .extend([sort(start, end).to_interval()]);
}

fn has_overlap(
  border_segments: &BTreeMap<u64, IntervalSet<u64>>,
  range_min: u64,
  range_max: u64,
  line_min: u64,
  line_max: u64,
) -> bool {
  if range_min > range_max {
    return false;
  }
  for (_, segments) in border_segments.range(range_min..=range_max) {
    if segments
      .iter()
      .any(|segment| segment.overlap(&Interval::new(line_min, line_max)))
    {
      return true;
    }
  }

  false
}

fn part_2(points: &Vec<(u64, u64)>) -> u64 {
  let mut horizontal_lines: BTreeMap<u64, IntervalSet<u64>> = BTreeMap::new();
  let mut vertical_lines: BTreeMap<u64, IntervalSet<u64>> = BTreeMap::new();
  for (i, &(x1, y1)) in points.iter().enumerate() {
    let (x2, y2) = points[(i + 1) % points.len()];
    if x1 == x2 {
      add_border_segment(&mut vertical_lines, x1, y1, y2);
    } else if y1 == y2 {
      add_border_segment(&mut horizontal_lines, y1, x1, x2);
    } else {
      panic!("Expected all lines to be vertical or horizontal");
    }
  }

  let mut max = 0;
  for (i, &(x1, y1)) in points.iter().enumerate() {
    for &(x2, y2) in points.iter().skip(i + 1) {
      let area = (x1.abs_diff(x2) + 1) * (y1.abs_diff(y2) + 1);
      if area > max {
        let (x1, x2) = sort_and_inset(x1, x2);
        let (y1, y2) = sort_and_inset(y1, y2);
        if !has_overlap(&vertical_lines, x1, x2, y1, y2)
          && !has_overlap(&horizontal_lines, y1, y2, x1, x2)
        {
          max = area;
        }
      }
    }
  }

  max
}

#[cfg(test)]
mod tests {
  use super::*;
  use indoc::indoc;

  const SAMPLE_INPUT: &str = indoc! {"
    7,1
    11,1
    11,7
    9,7
    9,5
    2,5
    2,3
    7,3
  "};

  #[test]
  fn test_part1() {
    assert_eq!(part_1(&parse(SAMPLE_INPUT)), 50);
  }

  #[test]
  fn test_part2() {
    assert_eq!(part_2(&parse(SAMPLE_INPUT)), 24);
  }
}
