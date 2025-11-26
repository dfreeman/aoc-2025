use aoc::prelude::*;
use nom::{
    character::complete::{space1, u8},
    multi::separated_list1,
};

aoc::solution! {
    year: 2024,
    day: 2,
    part_1,
    part_2,
}

fn part_1(input: &str) -> usize {
    parse(input).iter().filter(|report| is_safe(report)).count()
}

fn part_2(input: &str) -> usize {
    parse(input)
        .iter()
        .filter(|report| is_safe_with_dampener(report))
        .count()
}

fn parse(input: &str) -> Vec<Vec<u8>> {
    input.trim().parse_lines(separated_list1(space1, u8))
}

fn is_increasing(report: &[u8]) -> bool {
    let mut delta = 0;
    for pair in report.windows(2).take(3) {
        delta += if pair[0] < pair[1] { 1 } else { -1 };
    }
    delta > 0
}

fn unsafe_index(report: &[u8]) -> Option<usize> {
    let increasing = is_increasing(report);
    for (i, pair) in report.windows(2).enumerate() {
        let a = pair[0];
        let b = pair[1];
        if (a < b) != increasing || a == b || a.abs_diff(b) > 3 {
            return Some(i);
        }
    }
    None
}

fn is_safe(report: &[u8]) -> bool {
    unsafe_index(report).is_none()
}

fn without(report: &[u8], idx: usize) -> Vec<u8> {
    let mut result = Vec::with_capacity(report.len() - 1);
    result.extend(&report[..idx]);
    result.extend(&report[(idx + 1)..]);
    result
}

fn is_safe_with_dampener(report: &[u8]) -> bool {
    match unsafe_index(report) {
        None => true,
        Some(idx) => is_safe(&without(report, idx)) || is_safe(&without(report, idx + 1)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE_INPUT: &str = indoc! {"
        7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part_1(SAMPLE_INPUT), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part_2(SAMPLE_INPUT), 4);
    }
}
