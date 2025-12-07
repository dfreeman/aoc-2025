use std::collections::HashMap;

use aoc::{
    grid::{Direction, Grid},
    prelude::*,
};

solution! {
    year: 2025,
    day: 7,
    parse,
    part_1,
    part_2,
}

type Layout = (isize, Grid<bool>);

fn parse(input: &str) -> Layout {
    let start = input.find('S').expect("should have a starting position");
    let grid = Grid::new(
        input
            .parse_char_grid()
            .iter()
            .map(|row| row.iter().map(|&c| c == '^').collect())
            .collect(),
    );
    (start as isize, grid)
}

fn part_1((start, grid): &Layout) -> u64 {
    let mut splits = 0;
    let mut cells = vec![grid.cell((0, *start))];
    while !cells.is_empty() {
        let mut new_cells = vec![];
        for cell in cells {
            use Direction::*;
            if let Some(next) = cell.neighbour(S) {
                if *next {
                    splits += 1;
                    for neighbour in [W, E].into_iter().filter_map(|dir| next.neighbour(dir)) {
                        if !new_cells.contains(&neighbour) {
                            new_cells.push(neighbour);
                        }
                    }
                } else if !new_cells.contains(&next) {
                    new_cells.push(next);
                }
            }
        }
        cells = new_cells;
    }
    splits
}

fn part_2((start, grid): &Layout) -> u64 {
    let mut paths = 1;
    let mut cells = HashMap::from([(grid.cell((0, *start)), 1u64)]);
    while !cells.is_empty() {
        let mut new_cells = HashMap::new();
        for (cell, count) in cells {
            use Direction::*;
            if let Some(next) = cell.neighbour(S) {
                if *next {
                    let w = next
                        .neighbour(W)
                        .map(|w| *new_cells.entry(w).or_default() += count);
                    let e = next
                        .neighbour(E)
                        .map(|e| *new_cells.entry(e).or_default() += count);

                    // If both directions exist, then we've forked: that means each path
                    // that came here has generated an additional one as well.
                    if w.is_some() && e.is_some() {
                        paths += count;
                    }
                } else {
                    *new_cells.entry(next).or_default() += count;
                }
            }
        }
        cells = new_cells;
    }
    paths
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE_INPUT: &str = indoc! {"
        .......S.......
        ...............
        .......^.......
        ...............
        ......^.^......
        ...............
        .....^.^.^.....
        ...............
        ....^.^...^....
        ...............
        ...^.^...^.^...
        ...............
        ..^...^.....^..
        ...............
        .^.^.^.^.^...^.
        ...............
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part_1(&parse(SAMPLE_INPUT)), 21);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part_2(&parse(SAMPLE_INPUT)), 40);
    }
}
