use std::ops::{Deref, Index, IndexMut};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Coord {
    pub row: isize,
    pub col: isize,
}

impl Coord {
    pub fn translate(&self, direction: impl Into<Direction>) -> Coord {
        match direction.into() {
            Direction::N => Coord {
                row: self.row - 1,
                col: self.col,
            },
            Direction::NE => Coord {
                row: self.row - 1,
                col: self.col + 1,
            },
            Direction::E => Coord {
                row: self.row,
                col: self.col + 1,
            },
            Direction::SE => Coord {
                row: self.row + 1,
                col: self.col + 1,
            },
            Direction::S => Coord {
                row: self.row + 1,
                col: self.col,
            },
            Direction::SW => Coord {
                row: self.row + 1,
                col: self.col - 1,
            },
            Direction::W => Coord {
                row: self.row,
                col: self.col - 1,
            },
            Direction::NW => Coord {
                row: self.row - 1,
                col: self.col - 1,
            },
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl Direction {
    pub fn all() -> impl Iterator<Item = Direction> {
        use Direction::*;
        [N, NE, E, SE, S, SW, W, NW].into_iter()
    }

    pub fn cardinals() -> impl Iterator<Item = Direction> {
        use Direction::*;
        [N, E, S, W].into_iter()
    }
}

pub struct GridCell<'a, T> {
    coord: Coord,
    source: &'a Grid<T>,
}

impl<T> Copy for GridCell<'_, T> {}
impl<T> Clone for GridCell<'_, T> {
    fn clone(&self) -> Self {
        Self {
            coord: self.coord,
            source: self.source,
        }
    }
}

impl<'a, T> GridCell<'a, T> {
    pub fn neighbour(&self, direction: Direction) -> Option<GridCell<'a, T>> {
        let new_coord = self.coord.translate(direction);
        if new_coord.row < self.source.min_row
            || new_coord.col < self.source.min_col
            || new_coord.row > self.source.max_row
            || new_coord.col > self.source.max_col
        {
            None
        } else {
            Some(GridCell {
                coord: new_coord,
                source: self.source,
            })
        }
    }

    pub fn travel(&self, direction: impl Into<Direction>) -> impl Iterator<Item = GridCell<'a, T>> {
        let mut current = *self;
        let direction = direction.into();
        std::iter::from_fn(move || {
            let next = current.neighbour(direction);
            match next {
                Some(next) => {
                    current = next;
                    Some(current)
                }
                None => None,
            }
        })
    }

    pub fn value(&self) -> &T {
        &self.source[self.coord]
    }
}

impl<T> Deref for GridCell<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.value()
    }
}

#[derive(Clone)]
pub struct Grid<T> {
    data: Vec<T>,
    min_row: isize,
    max_row: isize,
    min_col: isize,
    max_col: isize,
}

impl<T> Grid<T> {
    pub fn new(cells: Vec<Vec<T>>) -> Self {
        let rows = cells.len();
        assert!(rows > 0);
        let cols = cells[0].len();
        let mut data = Vec::with_capacity(rows * cols);
        for row in cells {
            assert!(row.len() == cols);
            data.extend(row)
        }
        Self {
            data,
            min_row: 0,
            max_row: rows as isize - 1,
            min_col: 0,
            max_col: cols as isize - 1,
        }
    }

    pub fn cell(&self, coord: Coord) -> GridCell<'_, T> {
        GridCell {
            coord,
            source: self,
        }
    }

    pub fn coords(&self) -> impl Iterator<Item = Coord> + 'static {
        let rows = self.min_row..=self.max_row;
        let cols = self.min_col..=self.max_col;
        rows.flat_map(move |row| cols.clone().map(move |col| Coord { row, col }))
    }

    pub fn cells(&self) -> impl Iterator<Item = GridCell<'_, T>> {
        self.coords().map(|coord| GridCell {
            coord,
            source: self,
        })
    }

    fn coord_offset(&self, Coord { row, col }: Coord) -> usize {
        assert!(row >= self.min_row && row <= self.max_row);
        assert!(col >= self.min_col && col <= self.max_col);
        (row - self.min_row) as usize * (self.max_col - self.min_col + 1) as usize
            + (col - self.min_col) as usize
    }
}

impl<T> Index<Coord> for Grid<T> {
    type Output = T;

    fn index(&self, coord: Coord) -> &Self::Output {
        &self.data[self.coord_offset(coord)]
    }
}

impl<T> IndexMut<Coord> for Grid<T> {
    fn index_mut(&mut self, coord: Coord) -> &mut Self::Output {
        let offset = self.coord_offset(coord);
        &mut self.data[offset]
    }
}
