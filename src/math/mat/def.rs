pub use super::data::Data;

#[derive(Clone)]
pub struct Matrix<const R: usize, const C: usize> { 
    pub(super) cols: Data<R, C>
}