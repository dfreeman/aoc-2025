use aoc::prelude::*;

use itertools::Itertools;
use nom::{
    character::complete::{newline, space1, usize},
    multi::separated_list0,
    sequence::separated_pair,
};

aoc::solution! {
    day: 1,
    year: 2024,
    part_1,
    part_2,
}

fn part_1(input: &str) -> usize {
    let (mut list1, mut list2) = parse(input);
    list1.sort();
    list2.sort();
    list1.iter().zip(list2).map(|(a, b)| a.abs_diff(b)).sum()
}

fn part_2(input: &str) -> usize {
    let (first, second): (Vec<usize>, Vec<usize>) = parse(input);
    let freqs = second.into_iter().counts();
    first
        .iter()
        .filter_map(|n| freqs.get(&n).map(|m| n * m))
        .sum()
}

fn parse(input: &str) -> (Vec<usize>, Vec<usize>) {
    input
        .trim()
        .parse_full(separated_list0(
            newline,
            separated_pair(usize, space1, usize),
        ))
        .into_iter()
        .unzip()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE_INPUT: &str = indoc! {"
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part_1(SAMPLE_INPUT), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part_2(SAMPLE_INPUT), 31);
    }
}
