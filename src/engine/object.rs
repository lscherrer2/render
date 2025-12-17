use super::Model;
use super::Transform;

pub(super) trait Object {
    fn get_transform(&self) -> Transform;
    fn set_transform(&mut self, tf: Transform);
    fn get_transform_ref_mut(&mut self) -> &mut Transform;
}
