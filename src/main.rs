use std::path::Path;
use clap::{Arg, App};
use gltf;

mod data;
mod skelton;
mod tree;

use crate::data::GltfData;
use crate::tree::print_tree;
use crate::skelton::Skelton;

pub fn main() {
    let matches = App::new("po")
        .arg(Arg::with_name("gltf")
            .value_name("FILE")
            .help("Gltf file to load")
            .takes_value(true)
            .required(true))
        .get_matches();
    let (doc, buffers, images) = gltf::import(Path::new(matches.value_of("gltf").unwrap()))
        .expect("Import error");
    let gltfdata = GltfData {doc, buffers, images};
    print_tree(&gltfdata);
}