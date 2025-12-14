use super::{Matrix, data::Data};

impl<const R: usize, const C: usize> 
Matrix<R, C> {
    pub fn new(cols: [[f32; R]; C]) -> Self { Matrix{cols: Data::STACK(cols)} }
    pub fn new_stack(cols: [[f32; R]; C]) -> Self { Matrix{cols: Data::STACK(cols)} }
    pub fn new_heap(cols: [[f32; R]; C]) -> Self { Matrix{cols: Data::HEAP(Box::new(cols))} }
    pub fn from_rows(rows: [[f32; C]; R]) -> Self {
        let mut cols = [[0.0; R]; C];
        (0..C).for_each(|c| (0..R).for_each(|r| cols[c][r] = rows[r][c]));
        Self::new(cols)
    }
    pub fn from_cols(cols: [[f32; R]; C]) -> Self {
        Self::new(cols)
    }
    pub fn zeros() -> Self { Matrix{cols: Data::STACK([[0.0; R]; C])} }
    pub fn zeros_stack() -> Self { Matrix{cols: Data::STACK([[0.0; R]; C])} }
    pub fn zeros_heap() -> Self { Matrix{cols: Data::HEAP(Box::new([[0.0; R]; C]))} }
}