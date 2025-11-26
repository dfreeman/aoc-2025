use aoc::{
    grid::{Direction, Grid, GridCell},
    prelude::ParseExts,
};

aoc::solution! {
    year: 2024,
    day: 4,
    part_1,
    part_2,
}

fn part_1(input: &str) -> usize {
    let grid = Grid::new(input.parse_char_grid());
    let mut total = 0;
    for cell in grid.iter().filter(|c| **c == 'X') {
        'dir: for dir in Direction::all() {
            let mut cell = cell;
            for char in ['M', 'A', 'S'] {
                match cell.neighbour(dir) {
                    Some(next) if *next == char => cell = next,
                    _ => continue 'dir,
                }
            }
            total += 1;
        }
    }
    total
}

fn is_m_s(a: Option<GridCell<char>>, b: Option<GridCell<char>>) -> bool {
    match (a, b) {
        (Some(a), Some(b)) => *a == 'M' && *b == 'S' || *a == 'S' && *b == 'M',
        _ => false,
    }
}

fn part_2(input: &str) -> usize {
    use Direction::*;

    let grid = Grid::new(input.parse_char_grid());
    let mut total = 0;
    for cell in grid.iter().filter(|c| **c == 'A') {
        if is_m_s(cell.neighbour(NE), cell.neighbour(SW))
            && is_m_s(cell.neighbour(NW), cell.neighbour(SE))
        {
            total += 1;
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE_INPUT: &str = indoc! {"
        MMMSXXMASM
        MSAMXMSMSA
        AMXSXMAAMM
        MSAMASMSMX
        XMASAMXAMM
        XXAMMXXAMA
        SMSMSASXSS
        SAXAMASAAA
        MAMMMXMMMM
        MXMXAXMASX
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part_1(SAMPLE_INPUT), 18);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part_2(SAMPLE_INPUT), 9);
    }
}
