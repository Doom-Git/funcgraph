mod graph;
mod input;
mod parser;

use graph::gen_graph;
use parser::builder::create_tree;

use crate::parser::specs::java::JavaSpec;
use crate::parser::specs::python::PythonSpec;
use crate::parser::specs::rust::RustSpec;

use tree_sitter_java::LANGUAGE as JAVA;
use tree_sitter_python::LANGUAGE as PYTHON;
use tree_sitter_rust::LANGUAGE as RUST;

fn main() {
    let (lang, dir) = input::parse_input();

    let graph = match lang.as_str() {
        "rust" => create_tree::<RustSpec>(dir, RUST.into()),
        "java" => create_tree::<JavaSpec>(dir, JAVA.into()),
        "python" => create_tree::<PythonSpec>(dir, PYTHON.into()),
        _ => {
            eprintln!("Unsupported language: {}", lang);
            return;
        }
    };

    gen_graph(graph);
}
