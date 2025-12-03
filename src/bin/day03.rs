use itertools::Itertools;
use std::ops::RangeBounds;

aoc::solution! {
    year: 2025,
    day: 3,
    part_1,
    part_2,
}

fn parse(input: &str) -> Vec<Vec<u64>> {
    input
        .lines()
        .map(|l| l.chars().map(|c| c as u64 - '0' as u64).collect())
        .collect()
}

fn max_in<T: Ord>(els: &Vec<T>, range: impl RangeBounds<usize>) -> (usize, &T) {
    use std::ops::Bound::*;

    let offset_start = match range.start_bound() {
        Included(&n) => n,
        Excluded(&n) => n + 1,
        Unbounded => 0,
    };

    let offset_end = match range.end_bound() {
        Included(&n) => els.len() - n - 1,
        Excluded(&n) => els.len() - n,
        Unbounded => 0,
    };

    els.iter()
        .enumerate()
        .dropping(offset_start)
        .dropping_back(offset_end)
        .rev() // `max_by` returns the last max, and we want the first
        .max_by(|&(_, a), &(_, b)| a.cmp(b))
        .unwrap()
}

fn part_1(input: &str) -> u64 {
    let mut sum = 0;
    for row in parse(input) {
        let (idx, lhs) = max_in(&row, ..(row.len() - 1));
        let (_, rhs) = max_in(&row, (idx + 1)..);
        sum += (10 * lhs) + rhs;
    }
    sum
}

fn part_2(input: &str) -> u64 {
    let mut sum = 0;
    for row in parse(input) {
        let mut index = 0;
        let mut row_sum = 0;
        for place in (0..12).rev() {
            let (idx, val) = max_in(&row, index..(row.len() - place));
            row_sum += 10u64.pow(place as u32) * val;
            index = idx + 1;
        }
        sum += row_sum;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE_INPUT: &str = indoc! {"
        987654321111111
        811111111111119
        234234234234278
        818181911112111
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part_1(SAMPLE_INPUT), 357);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part_2(SAMPLE_INPUT), 3121910778619);
    }
}
