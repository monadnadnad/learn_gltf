use cgmath::Matrix4;
use crate::data::GltfData;
pub struct Joint {
    name: Option<String>,
    inverse_bind_matrix: Matrix4<f32>,
    parent: Box<Joint>
}

pub struct Skelton {
    roots: Vec<Box<Joint>>
}

impl Skelton {
    pub fn from_gltf(gltf: &GltfData) -> Self {
        for skin in gltf.doc.skins() {
            for node in skin.joints() {
                println!("{:?}", node.mesh());
            }
            for accessor in skin.inverse_bind_matrices() {
                println!("{:?}", accessor);
            }
        }
        Skelton { roots: vec![] }
    }
}