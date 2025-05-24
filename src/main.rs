mod parser;
mod input;
mod graph;

use parser::builder::create_tree;
use parser::java::JavaSpec;
use parser::rust::RustSpec;
use graph::gen_graph;



fn main() {
    let (lang, dir) = input::parse_input();

    let graph = match lang.as_str() {
        "rust" => create_tree::<RustSpec>(dir, tree_sitter_rust::LANGUAGE.into()),
        // "java" => create_tree::<JavaSpec>(path, tree_sitter_java::language()),
        _ => {
            eprintln!("Unsupported language: {}", lang);
            return;
        }
    };

    gen_graph(graph);
}