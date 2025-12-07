use aoc::prelude::*;
use std::ops::RangeBounds;

solution! {
    year: 2025,
    day: 3,
    parse,
    part_1,
    part_2,
}

fn parse(input: &str) -> Vec<Vec<u64>> {
    input.parse_char_grid().map_2d(|c| c as u64 - '0' as u64)
}

fn max_in<T: Ord>(els: &Vec<T>, range: impl RangeBounds<usize>) -> (usize, &T) {
    use std::ops::Bound::*;

    let skip_start = match range.start_bound() {
        Included(&n) => n,
        Excluded(&n) => n + 1,
        Unbounded => 0,
    };

    let skip_end = match range.end_bound() {
        Included(&n) => els.len() - n - 1,
        Excluded(&n) => els.len() - n,
        Unbounded => 0,
    };

    els.iter()
        .enumerate()
        .skip(skip_start)
        .rev()
        .skip(skip_end)
        .max_by(|&(_, a), &(_, b)| a.cmp(b))
        .expect("max_in called with an empty Vec")
}

fn part_1(input: &Vec<Vec<u64>>) -> u64 {
    let mut sum = 0;
    for row in input {
        let (idx, lhs) = max_in(&row, ..(row.len() - 1));
        let (_, rhs) = max_in(&row, (idx + 1)..);
        sum += (10 * lhs) + rhs;
    }
    sum
}

fn part_2(input: &Vec<Vec<u64>>) -> u64 {
    let mut sum = 0;
    for row in input {
        let mut start = 0;
        for place in (0..12).rev() {
            let end = row.len() - place;
            let (idx, val) = max_in(&row, start..end);
            sum += 10u64.pow(place as u32) * val;
            start = idx + 1;
        }
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
        assert_eq!(part_1(&parse(SAMPLE_INPUT)), 357);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part_2(&parse(SAMPLE_INPUT)), 3121910778619);
    }
}
