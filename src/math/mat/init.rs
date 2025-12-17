use super::{Matrix, data::Data};
use std::mem::size_of;

impl<const R: usize, const C: usize> Matrix<R, C> {
    pub fn new(cols: [[f32; R]; C]) -> Self {
        Matrix {
            cols: if R * C * size_of::<f32>() <= 256 {
                Data::STACK(cols)
            } else {
                Data::HEAP(Box::new(cols))
            },
        }
    }
    pub fn zeros() -> Self {
        Self::new([[0.0; R]; C])
    }
    pub fn from_cols(cols: [[f32; R]; C]) -> Self {
        Self::new(cols)
    }
    pub fn from_rows(rows: [[f32; C]; R]) -> Self {
        let mut res = Self::zeros();
        (0..C).for_each(|c| (0..R).for_each(|r| res[(r, c)] = rows[r][c]));
        res
    }
}

impl<const D: usize> Matrix<D, D> {
    pub fn identity() -> Self {
        let mut res = Self::zeros();
        (0..D).for_each(|idx| res[(idx, idx)] = 1.0);
        res
    }
}
