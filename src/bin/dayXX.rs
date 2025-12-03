use aoc::prelude::*;

aoc::solution! {
    year: 2025,
    day: 000000,
    parse,
    part_1,
    part_2,
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE_INPUT: &str = indoc! {"

    "};

    #[test]
    fn test_part1() {
        assert_eq!(part_1(&parse(SAMPLE_INPUT)), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part_2(&parse(SAMPLE_INPUT)), 0);
    }
}
