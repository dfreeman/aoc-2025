use std::cell::Cell;

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

struct QuantumNonsense {
  is_splitter: bool,
  paths: Cell<u64>,
}

impl QuantumNonsense {
  fn new(is_splitter: bool) -> Self {
    Self {
      is_splitter,
      paths: Cell::new(0),
    }
  }

  fn increment_paths(&self, count: u64) {
    let current = self.paths.get();
    self.paths.set(current + count);
  }

  fn paths(&self) -> u64 {
    self.paths.get()
  }
}

type Layout = (isize, Grid<bool>);

fn parse(input: &str) -> Layout {
  let start = input.find('S').expect("should have a starting position");
  let grid = Grid::new(input.parse_char_grid().map_2d(|c| c == '^'));
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
  let grid = grid.map(|c| QuantumNonsense::new(*c));

  grid[(0, *start)].increment_paths(1);

  for cell in grid.cells().filter(|c| c.paths() > 0) {
    if let Some(next) = cell.neighbour(Direction::S) {
      if next.is_splitter {
        for dir in [Direction::W, Direction::E] {
          if let Some(neighbour) = next.neighbour(dir) {
            neighbour.increment_paths(cell.paths());
          }
        }
      } else {
        next.increment_paths(cell.paths());
      }
    }
  }

  grid.rows().last().unwrap().map(|c| c.paths()).sum()
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
