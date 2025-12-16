#[derive(Clone)]
pub enum Data<const R: usize, const C: usize> {
    HEAP(Box<[[f32; R]; C]>),
    STACK([[f32; R]; C]),
}
