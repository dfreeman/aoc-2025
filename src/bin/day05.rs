use aoc::prelude::*;

solution! {
  year: 2025,
  day: 5,
  parse,
  part_1,
  part_2,
}

type Parsed = (Vec<(u64, u64)>, Vec<u64>);

fn parse(input: &str) -> Parsed {
  use parse::*;

  input.parse_full(separated_pair(
    separated_list1(newline, separated_pair(u64, char('-'), u64)),
    tag("\n\n"),
    separated_list1(newline, u64),
  ))
}

fn part_1((ranges, ids): Parsed) -> usize {
  ids
    .iter()
    .filter(|&id| {
      ranges
        .iter()
        .any(|&(start, end)| (start..=end).contains(id))
    })
    .count()
}

fn part_2((mut ranges, _): Parsed) -> u64 {
  // Sorting ensures any ranges that full subsume others are processed first
  ranges.sort_by(|&a, &b| (b.1 - b.0).cmp(&(a.1 - a.0)));

  let mut intervals = Vec::new();
  for &(mut start, mut end) in &ranges {
    for &(prev_start, prev_end) in &intervals {
      if start >= prev_start && start <= prev_end {
        start = prev_end + 1;
      }
      if end <= prev_end && end >= prev_start {
        end = prev_start - 1;
      }
    }
    if end >= start {
      intervals.push((start, end));
    }
  }

  intervals.iter().map(|(start, end)| end - start + 1).sum()
}

#[cfg(test)]
mod tests {
  use super::*;
  use indoc::indoc;

  const SAMPLE_INPUT: &str = indoc! {"
    3-5
    10-14
    16-20
    12-18

    1
    5
    8
    11
    17
    32
  "};

  #[test]
  fn test_part1() {
    assert_eq!(part_1(parse(SAMPLE_INPUT)), 3);
  }

  #[test]
  fn test_part2() {
    assert_eq!(part_2(parse(SAMPLE_INPUT)), 14);
  }
}
