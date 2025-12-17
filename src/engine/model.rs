use super::Object;
use super::Transform;
use crate::math::Vec3;

use tobj::{LoadOptions, LoadResult};

pub struct Model {
    tf: Transform,
    triangles: Vec<[Vec3; 3]>,
}
struct ObjLoadError;
impl Model {
    pub fn from_obj(fp: String) -> Result<Self, ObjLoadError> {
        let opts = LoadOptions {
            triangulate: true,
            single_index: true,
            ..LoadOptions::default()
        };
        let (models, ..) = tobj::load_obj(fp, &opts).map_err(|e| ObjLoadError)?;

        let mut triangles: Vec<[Vec3; 3]> = Vec::new();
        models.iter().for_each(|model| {
            let mesh = &model.mesh;
            let idx = |i| i as usize;
            let get = |i| {
                [
                    mesh.positions[3 * i],
                    mesh.positions[3 * i + 1],
                    mesh.positions[3 * i + 2],
                ]
            };
            mesh.indices.chunks_exact(3).for_each(|tri| {
                triangles.push([
                    get(idx(tri[0])).into(),
                    get(idx(tri[1])).into(),
                    get(idx(tri[2])).into(),
                ]);
            })
        });
        Ok(Self {
            tf: Transform::default(),
            triangles,
        })
    }
}
impl Object for Model {
    fn get_transform(&self) -> Transform {
        self.tf
    }
    fn set_transform(&mut self, tf: Transform) {
        self.tf = tf;
    }
    fn get_transform_ref_mut(&mut self) -> &mut Transform {
        &mut self.tf
    }
}
