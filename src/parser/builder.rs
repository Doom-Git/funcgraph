use super::specs::language_spec::LanguageSpec;
use crate::parser::extractor::extract_structure;
use std::collections::HashMap;
use std::path::PathBuf;
use tree_sitter::{Language, Parser};
use walkdir::{DirEntry, WalkDir};

pub fn create_tree<T: LanguageSpec>(
    file_path: PathBuf,
    ts_language: Language,
) -> HashMap<String, Vec<String>> {
    let mut parser = Parser::new();

    parser
        .set_language(&ts_language)
        .expect("Failed to set language");

    let (mut pay1, mut pay2, mut pay3, mut pay4, mut pay5) = ("", "", "", "", "");

    WalkDir::new(&file_path)
        .into_iter()
        .filter_entry(|e| is_src_file::<T>(e))
        .filter_map(|v| v.ok())
        .for_each(|x| println!("{}", x.path().display()));

    let content = std::fs::read_to_string(&file_path).expect("File read failed");

    let tree = parser.parse(&content, None).expect("Parsing failed");

    extract_structure::<T>(tree.root_node(), &content, String::new())
}

fn is_src_file<T: LanguageSpec>(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| entry.depth() == 0 || s.ends_with(T::file_type()))
        .unwrap_or(false)
}
