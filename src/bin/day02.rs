use aoc::prelude::*;
use nom::{
    character::complete::{char, u64},
    multi::separated_list1,
    sequence::separated_pair,
};

aoc::solution! {
    year: 2025,
    day: 2,
    part_1,
    part_2,
}

fn parse(input: &str) -> Vec<(u64, u64)> {
    input.parse_full(separated_list1(
        char(','),
        separated_pair(u64, char('-'), u64),
    ))
}

/// "Oh cool, I can just do this with math!"
fn part_1(input: &str) -> u64 {
    parse(input)
        .iter()
        .flat_map(|&(start, end)| {
            let start_len = start.digit_count();
            let start_half = if start_len.is_multiple_of(2) {
                let digits = 10u64.pow(start_len / 2);
                let first = start / digits;
                let second = start % digits;
                first + (first < second) as u64
            } else {
                10u64.pow((start_len + 1) / 2 - 1)
            };
            let end_len = end.digit_count();
            let end_half = if end_len.is_multiple_of(2) {
                let digits = 10u64.pow(end_len / 2);
                let first = end / digits;
                let second = end % digits;
                first - (first > second) as u64
            } else {
                10u64.pow((end_len - 1) / 2) - 1
            };
            start_half..=end_half
        })
        .map(|half| (half * 10u64.pow(half.digit_count())) + half)
        .sum()
}

fn split(n: u64, parts: u32) -> Option<Vec<u64>> {
    let len = n.digit_count();
    if len.is_multiple_of(parts) {
        Some(
            (0..parts)
                .map(|c| {
                    let shifter = 10u64.pow(len - ((c + 1) * (len / parts)));
                    (n / shifter) % 10u64.pow(len / parts)
                })
                .collect(),
        )
    } else {
        None
    }
}

/// "...actually, brute force is sounding pretty good right now."
fn part_2(input: &str) -> u64 {
    parse(input)
        .iter()
        .map(|&(start, end)| {
            let mut total = 0;
            for n in start..=end {
                let len = n.digit_count();
                for parts in 2..=len {
                    if let Some(splits) = split(n, parts) {
                        let first = splits[0];
                        if splits.iter().all(|&part| part == first) {
                            total += n;
                            break;
                        }
                    }
                }
            }
            total
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE_INPUT: &str = indoc! {"
        11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part_1(SAMPLE_INPUT), 1227775554);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part_2(SAMPLE_INPUT), 4174379265);
    }
}
