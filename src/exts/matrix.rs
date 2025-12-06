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
