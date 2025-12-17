use super::Matrix;
use std::{
    iter::{IntoIterator, Iterator},
    ops::{Index, IndexMut},
};

struct RowMajorIter<'a, const R: usize, const C: usize> {
    mat: &'a Matrix<R, C>,
    idx: usize,
}
struct ColMajorIter<'a, const R: usize, const C: usize> {
    mat: &'a Matrix<R, C>,
    idx: usize,
}

impl<'a, const R: usize, const C: usize> Iterator for RowMajorIter<'a, R, C> {
    type Item = ((usize, usize), f32);
    fn next(&mut self) -> Option<Self::Item> {
        if (self.idx < R * C) {
            let mat_idx = (self.idx % C, self.idx);
            let r = self.mat.index(mat_idx);
            self.idx += 1;
            Option::Some((mat_idx, *r))
        } else {
            Option::None
        }
    }
}
impl<'a, const R: usize, const C: usize> Iterator for ColMajorIter<'a, R, C> {
    type Item = ((usize, usize), f32);
    fn next(&mut self) -> Option<Self::Item> {
        if (self.idx < R * C) {
            let mat_idx = (self.idx, self.idx % R);
            let r = self.mat.index(mat_idx);
            self.idx += 1;
            Option::Some((mat_idx, *r))
        } else {
            Option::None
        }
    }
}
