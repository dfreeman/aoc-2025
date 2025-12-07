pub trait Transpose<T: Copy> {
    fn transpose(&self) -> Vec<Vec<T>>;
}

impl<T: Copy> Transpose<T> for Vec<Vec<T>> {
    fn transpose(&self) -> Vec<Vec<T>> {
        if self.is_empty() {
            return vec![];
        }

        let rows = self.len();
        let cols = self[0].len();
        let mut transposed = vec![vec![self[0][0]; rows]; cols];
        for r in 0..rows {
            for c in 0..cols {
                transposed[c][r] = self[r][c];
            }
        }
        transposed
    }
}

pub trait Map2D {
    type Item;
    type Output<U>;

    fn map_2d<U, F>(self, f: F) -> Self::Output<U>
    where
        F: Fn(Self::Item) -> U;
}

impl<R, C, T> Map2D for R
where
    R: IntoIterator<Item = C>,
    C: IntoIterator<Item = T>,
{
    type Item = T;
    type Output<U> = Vec<Vec<U>>;

    fn map_2d<U, F>(self, f: F) -> Self::Output<U>
    where
        F: Fn(Self::Item) -> U,
    {
        self.into_iter()
            .map(|row| row.into_iter().map(|el| f(el)).collect())
            .collect()
    }
}
