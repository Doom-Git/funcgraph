use std::path::PathBuf;
use std::collections::HashMap;
use tree_sitter::{Node, Parser};

pub fn create_tree(lang: String, dir: PathBuf) -> HashMap<String, Vec<String>> {
    let mut parser = Parser::new();
    // TODO: implement better language chooser
    parser
        .set_language(&tree_sitter_rust::LANGUAGE.into())
        .expect("Error loading Rust grammar!");

    let content = std::fs::read_to_string(dir).expect("File not found!");
    let tree = parser.parse(&content, None);
    return extract_actual_structure(tree.unwrap().root_node(), content);
}

// TODO: Find a way to make this faster

// TODO: Find a way to not use only strings

// TODO: Some languages differ in naming for some items
//? Solution: Use a match with variables for each different name etc.
fn extract_actual_structure(root: Node, content: String) -> HashMap<String, Vec<String>> {
    let mut cursor = root.walk();
    let mut found;
    let mut map: HashMap<_, _> = HashMap::new();

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
                if !map.contains_key(function_name) {
                    map.insert(function_name.to_string(), vec![called_fn_name.to_string()]);
                } else {
                    // If it gets here, the item was found
                    map.get_mut(function_name).unwrap().push(called_fn_name.to_string());
                }
            }
        });
    }

    return map;
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
