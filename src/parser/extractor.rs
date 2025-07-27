use super::specs::language_spec::LanguageSpec;
use std::collections::{HashMap, HashSet};
use tree_sitter::Node;

pub fn extract_structure<T: LanguageSpec>(
    root: Node,
    content: &str,
    class_name: String,
) -> HashMap<String, Vec<String>> {
    let mut cursor = root.walk();
    let mut map: HashMap<String, HashSet<String>> = HashMap::new();

    /*    for i in root.children(&mut cursor) {
            println!("{:?}", i);
        }
    */

    println!("{:?}", root.to_string());

    for node in root.children(&mut cursor).filter(T::is_function_node) {
        let name = T::get_function_name(&node, content).unwrap().to_string();
        map.entry(name.clone()).or_default();

        if let Some(body) = T::get_function_body(&node) {
            visit_node_iterative(body, |n| {
                if T::is_call_expression(&n) {
                    if let Some(called) = T::get_called_function_name(&n, content) {
                        map.entry(class_name.clone() + &name)
                            .or_default()
                            .insert(called.to_string());
                    }
                }
            });
        }
    }

    map.into_iter()
        .map(|(k, v)| (k, v.into_iter().collect()))
        .collect()
}

pub fn visit_node_iterative<F>(start: Node, mut on_node: F)
where
    F: FnMut(Node),
{
    let mut stack = vec![start];

    while let Some(node) = stack.pop() {
        on_node(node);

        for i in (0..node.named_child_count()).rev() {
            if let Some(child) = node.named_child(i) {
                stack.push(child);
            }
        }
    }
}
