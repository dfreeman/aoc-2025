use aoc::prelude::*;
use nom::{
    character::complete::{char, i32},
    sequence::pair,
};

aoc::solution! {
    year: 2025,
    day: 1,
    part_1,
    part_2,
}

#[derive(Debug)]
enum Dir {
    Left,
    Right,
}

impl Dir {
    fn as_i32(&self) -> i32 {
        match self {
            Dir::Left => -1,
            Dir::Right => 1,
        }
    }
}

fn parse(input: &str) -> Vec<(Dir, i32)> {
    let dir = char('L')
        .map(|_| Dir::Left)
        .or(char('R').map(|_| Dir::Right));

    input.parse_lines(pair(dir, i32))
}

fn part_1(input: &str) -> i32 {
    let mut zeroes: i32 = 0;
    let mut position: i32 = 50;
    for (dir, dist) in parse(input) {
        position += dir.as_i32() * dist;
        position %= 100;
        if position == 0 {
            zeroes += 1;
        }
    }
    zeroes
}

fn part_2(input: &str) -> i32 {
    let mut zeroes: i32 = 0;
    let mut position: i32 = 50;
    for (dir, mut dist) in parse(input) {
        // Account for any full spins
        zeroes += dist / 100;
        dist %= 100;
        if dist == 0 {
            continue;
        }

        // Check whether we cross zero one more time
        let start = position;
        position += dir.as_i32() * dist;
        if start != 0 && (position <= 0 || position >= 100) {
            zeroes += 1;
        }

        // Wrap back to 0..100
        position %= 100;
        if position < 0 {
            position += 100;
        }
    }
    zeroes
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE_INPUT: &str = indoc! {"
        L68
        L30
        R48
        L5
        R60
        L55
        L1
        L99
        R14
        L82
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part_1(SAMPLE_INPUT), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part_2(SAMPLE_INPUT), 6);
    }
}
