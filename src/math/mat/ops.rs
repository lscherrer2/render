use super::{Matrix, data::Data};

impl<const R: usize, const C: usize> std::ops::Index<(usize, usize)> for Matrix<R, C> {
    type Output = f32;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &(match &self.cols {
            Data::HEAP(a) => a,
            Data::STACK(a) => a,
        })[index.1][index.0]
    }
}

impl<const R: usize, const C: usize> std::ops::IndexMut<(usize, usize)> for Matrix<R, C> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut (match &mut self.cols {
            Data::HEAP(a) => a,
            Data::STACK(a) => a,
        })[index.1][index.0]
    }
}

impl<const R: usize, const C: usize, const N: usize> std::ops::Mul<Matrix<N, C>> for Matrix<R, N> {
    type Output = Matrix<R, C>;
    fn mul(self, rhs: Matrix<N, C>) -> Self::Output {
        let mut res: Matrix<R, C> = Matrix::zeros();
        (0..C).for_each(|c| {
            (0..R).for_each(|r| res[(r, c)] = (0..N).map(|n| self[(r, n)] * rhs[(n, c)]).sum())
        });
        res
    }
}
