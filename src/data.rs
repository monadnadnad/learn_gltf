use std::fs::File;
use std::path::Path;

pub struct GltfData {
    /// Helper for gltf
    pub doc: gltf::Document,
    pub buffers: Vec<gltf::buffer::Data>,
    pub images: Vec<gltf::image::Data>,
}

impl GltfData {
    fn export_json<P: AsRef<Path>>(self, path: P) -> Result<(), Box<dyn std::error::Error>>{
        let buffer = File::create(path)?;
        self.doc.into_json().to_writer_pretty(buffer)?;
        Ok(())
    }
}