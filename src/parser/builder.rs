use std::collections::HashMap;
use std::path::PathBuf;
use tree_sitter::{Parser, Language};
use crate::parser::extractor::extract_structure;
use super::specs::language_spec::LanguageSpec;

pub fn create_tree<T: LanguageSpec>(
    file_path: PathBuf,
    ts_language: Language,
) -> HashMap<String, Vec<String>> {
    let mut parser = Parser::new();

    parser.set_language(&ts_language).expect("Failed to set language");

    let content = std::fs::read_to_string(&file_path).expect("File read failed");

    let tree = parser.parse(&content, None).expect("Parsing failed");

    extract_structure::<T>(tree.root_node(), &content)
}
