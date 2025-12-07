use aoc::{
    grid::{Direction, Grid, GridCell},
    prelude::*,
};

solution! {
    year: 2025,
    day: 4,
    parse,
    part_1,
    part_2,
}

fn parse(input: &str) -> Grid<bool> {
    Grid::new(input.parse_char_grid().map_2d(|c| c == '@'))
}

fn occupied_neighbours(cell: &GridCell<bool>) -> usize {
    Direction::all()
        .filter(|&dir| {
            cell.neighbour(dir)
                .map(|neighbour| *neighbour)
                .unwrap_or(false)
        })
        .count()
}

fn part_1(grid: &Grid<bool>) -> usize {
    grid.cells()
        .filter(|&cell| *cell && occupied_neighbours(&cell) < 4)
        .count()
}

fn part_2(grid: &Grid<bool>) -> u64 {
    let mut grid = grid.clone();
    let mut count = 0;
    loop {
        let start = count;
        for coord in grid.coords() {
            if grid[coord] && occupied_neighbours(&grid.cell(coord)) < 4 {
                count += 1;
                grid[coord] = false;
            }
        }
        if start == count {
            break;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE_INPUT: &str = indoc! {"
        ..@@.@@@@.
        @@@.@.@.@@
        @@@@@.@.@@
        @.@@@@..@.
        @@.@@@@.@@
        .@@@@@@@.@
        .@.@.@.@@@
        @.@@@.@@@@
        .@@@@@@@@.
        @.@.@@@.@.
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part_1(&parse(SAMPLE_INPUT)), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part_2(&parse(SAMPLE_INPUT)), 43);
    }
}
