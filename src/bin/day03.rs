use regex::Regex;

aoc::solution! {
    year: 2024,
    day: 3,
    part_1,
    part_2,
}

enum Instr {
    Do,
    Dont,
    Mul(u32, u32),
}

fn part_1(input: &str) -> u32 {
    parse(input)
        .into_iter()
        .map(|instr| match instr {
            Instr::Mul(a, b) => a * b,
            _ => 0,
        })
        .sum()
}

fn part_2(input: &str) -> u32 {
    let mut acc = 0;
    let mut running = true;
    for instr in parse(input) {
        match instr {
            Instr::Mul(a, b) => {
                if running {
                    acc += a * b
                }
            }
            Instr::Do => running = true,
            Instr::Dont => running = false,
        }
    }
    acc
}

fn parse(input: &str) -> Vec<Instr> {
    Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)")
        .unwrap()
        .captures_iter(input)
        .map(|cap| match &cap[0] {
            "do()" => Instr::Do,
            "don't()" => Instr::Dont,
            _ => Instr::Mul(cap[1].parse().unwrap(), cap[2].parse().unwrap()),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE_INPUT_P1: &str = indoc! {"
        xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part_1(SAMPLE_INPUT_P1), 161);
    }

    const SAMPLE_INPUT_P2: &str = indoc! {"
        xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
    "};

    #[test]
    fn test_part2() {
        assert_eq!(part_2(SAMPLE_INPUT_P2), 48);
    }
}
