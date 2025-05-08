use std::path::PathBuf;
use tree_sitter::{Node, Parser};

pub fn create_tree(lang: String, dir: PathBuf) {
    let mut parser = Parser::new();
    // TODO: implement better language chooser
    parser
        .set_language(&tree_sitter_rust::LANGUAGE.into())
        .expect("Error loading Rust grammar!");

    let content = std::fs::read_to_string(dir).expect("File not found!");
    let tree = parser.parse(&content, None);
    // println!("{:?}", tree.as_ref().unwrap().root_node().to_sexp());
    extract_actual_structure(tree.unwrap().root_node(), content);
}

// TODO: Find a way to make this faster
fn extract_actual_structure(root: Node, content: String) {
    let mut cursor = root.walk();
    let mut found;

    for function in root
        .children(&mut cursor)
        .filter(|n| n.kind() == "function_item")
    {
        let name_node = function.child_by_field_name("name").unwrap();
        let body_node = function.child_by_field_name("body").unwrap();
        let function_name = &content[name_node.byte_range()];

        found = false;

        visit_node_iterative(body_node, |call_node| {
            if let Some(called_fn_node) = call_node.child_by_field_name("function") {
                let called_fn_name = &content[called_fn_node.byte_range()];
                found = true;
                println!("{function_name} calls {called_fn_name}");
            }
        });

        if !found {
            println!("{function_name} does not call any function");
        }
    }
}


fn visit_node_iterative<F>(start_node: Node, mut on_call: F)
where
    F: FnMut(Node),
{
    let mut stack = vec![start_node];

    while let Some(node) = stack.pop() {
        if node.kind() == "call_expression" {
            on_call(node);
        }

        // Push alle Kinder auf den Stack
        for i in (0..node.child_count()).rev() {
            if let Some(child) = node.child(i) {
                stack.push(child);
            }
        }
    }
}
