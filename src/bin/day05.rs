use std::collections::{HashMap, HashSet};

use aoc::prelude::*;
use nom::{
    bytes::complete::tag, character::complete::newline, character::complete::u32,
    multi::separated_list1, sequence::separated_pair,
};
use topological_sort::TopologicalSort;

aoc::solution! {
    year: 2024,
    day: 5,
    part_1,
    part_2,
}

struct Input {
    rules: HashMap<u32, Vec<u32>>,
    updates: Vec<Vec<u32>>,
}

fn parse(input: &str) -> Input {
    let (rule_pairs, updates) = input.parse_full(separated_pair(
        separated_list1(newline, separated_pair(u32, tag("|"), u32)),
        tag("\n\n"),
        separated_list1(newline, separated_list1(tag(","), u32)),
    ));

    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();
    for (from, to) in rule_pairs {
        rules.entry(from).or_default().push(to);
    }

    Input { rules, updates }
}

fn is_correct(rules: &HashMap<u32, Vec<u32>>, update: &[u32]) -> bool {
    let mut seen = HashSet::new();
    for page in update.iter() {
        if let Some(targets) = rules.get(page) {
            if targets.iter().any(|t| seen.contains(t)) {
                return false;
            }
        }
        seen.insert(page);
    }
    true
}

fn part_1(input: &str) -> u32 {
    let input = parse(input);
    input
        .updates
        .iter()
        .filter(|update| is_correct(&input.rules, update))
        .map(|update| update[update.len() / 2])
        .sum()
}

fn part_2(input: &str) -> u32 {
    let input = parse(input);
    input
        .updates
        .iter()
        .filter(|update| !is_correct(&input.rules, update))
        .map(|update| {
            let mut sort: TopologicalSort<u32> = TopologicalSort::new();
            for (from, tos) in &input.rules {
                if !update.contains(from) {
                    continue;
                }
                for to in tos {
                    if update.contains(to) {
                        sort.add_dependency(*to, *from);
                    }
                }
            }
            for page in update {
                sort.insert(*page);
            }

            let sorted = sort.collect::<Vec<_>>();
            sorted[sorted.len() / 2]
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE_INPUT: &str = indoc! {"
        47|53
        97|13
        97|61
        97|47
        75|29
        61|13
        75|53
        29|13
        97|29
        53|29
        61|53
        97|53
        61|29
        47|13
        75|47
        97|75
        47|61
        75|61
        47|29
        75|13
        53|13

        75,47,61,53,29
        97,61,53,29,13
        75,29,13
        75,97,47,61,53
        61,13,29
        97,13,75,29,47    
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part_1(SAMPLE_INPUT), 143);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part_2(SAMPLE_INPUT), 123);
    }
}
