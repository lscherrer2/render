use crate::config::Dtype;

pub struct Mat4 {
    pub data: [Dtype; 16],
}
impl Mat4 {
    pub fn new() -> Self {
        Mat4{data: [0 as Dtype; 16]}
    }
}

impl Mat4 {
    pub fn index_mut(&mut self, row: usize, col: usize) -> &mut Dtype {
        &mut self.data[row*4+col]
    }
    pub fn index(&self, row: usize, col: usize) -> &Dtype {
        &self.data[row * 4 + col]
    }
    pub fn multiply(lhs: &Mat4, rhs: &Mat4) -> Self {
        let mut res = Mat4::new();
        (0..4).for_each(|r| (0..4).for_each(|c| 
            *res.index_mut(r, c) = (0..4)
                .map(|i| lhs.index(r, i) * rhs.index(i, c))
                .sum()
        ));
        res
    }
    pub fn left_multiply(&mut self, other: &Mat4) -> &mut Self {
        let res = Self::multiply(self, other);
        self.data.copy_from_slice(&res.data);
        self
    }
}
