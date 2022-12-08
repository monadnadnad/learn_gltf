use gltf::*;
use crate::GltfData;

fn indent(depth: usize) -> String {
    " ".repeat(depth)
}

pub fn print_tree(gltf: &GltfData) {
    for scene in gltf.doc.scenes() {
        print!("Scene {}", scene.index());
        print!(" ({})", scene.name().unwrap_or("<Unnamed>"));
        println!();
        for node in scene.nodes() {
            print_node(&node, 1);
        }
    }
}

fn print_node(node: &Node, depth: usize) {
    println!("{}- Node {} ({})",
        indent(depth),
        node.index(),
        node.name().unwrap_or("<No Name>"));
    if let Some(mesh) = node.mesh() {
        print_mesh(&mesh, depth + 1);
    }

    for child in node.children() {
        print_node(&child, depth + 1);
    }
}

fn print_mesh(mesh: &Mesh, depth: usize) {
    println!("{}- Mesh {} ({})",
        indent(depth),
        mesh.index(),
        mesh.name().unwrap_or("<No Name>"));
    for p in mesh.primitives() {
        println!("{}- Attrs", indent(depth + 1));
        for a in p.attributes() {
            println!("{}- {:?}", indent(depth + 2), a.0);
            print_accessor(&a.1, depth + 2);
        }
    }
}

fn print_accessor(accessor: &Accessor, depth: usize) {
    println!("{}type: {:?}",
        indent(depth),
        accessor.data_type());
    if let Some(view) = accessor.view() {
        print_buffer_view(&view, depth + 1);
    }
}

fn print_buffer_view(view: &buffer::View, depth: usize) {
    println!("{}{:?}",
        indent(depth),
        match view.buffer().source() {
            buffer::Source::Bin => String::from("Bin"),
            buffer::Source::Uri(b64) => b64.to_string()
        });
}