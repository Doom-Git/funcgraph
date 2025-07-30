use super::specs::language_spec::LanguageSpec;
use rayon::prelude::*;
use std::collections::HashMap;
use tree_sitter::Node;

pub fn extract_structure<T: LanguageSpec>(
    root: Node,
    content: &str,
    class_name: String,
) -> HashMap<String, Vec<String>> {
    let mut cursor = root.walk();
    //let mut map: HashMap<String, Vec<String>> = HashMap::new();

    //    println!("{:?}", root.to_string());

    let partial_maps: Vec<HashMap<String, Vec<String>>> = root
        .children(&mut cursor)
        .par_bridge()
        .map(|node| {
            let mut local_map: HashMap<String, Vec<String>> = HashMap::new();

            if T::is_function_node(&node) {
                let name = T::get_node_name(&node, content).unwrap();
                let key = format!("{class_name}{name}");

                local_map.entry(key.clone()).or_default();

                if let Some(body) = T::get_node_body(&node) {
                    visit_node_iterative(body, |n| {
                        if T::is_call_expression(&n) {
                            if let Some(called) = T::get_called_function_name(&n, content) {
                                local_map
                                    .entry(key.clone())
                                    .or_default()
                                    .push(format!("{class_name}{called}"));
                            }
                        }
                    });
                }
            } else if T::is_class_node(&node) {
                if let (Some(class_name), Some(body)) =
                    (T::get_node_name(&node, content), T::get_node_body(&node))
                {
                    let child_map =
                        extract_structure::<T>(body, content, format!("{class_name}::"));
                    local_map.extend(child_map);
                }
            }

            local_map
        })
        .collect();

    let mut final_map: HashMap<String, Vec<String>> = HashMap::new();
    for m in partial_maps {
        for (k, mut v) in m {
            final_map.entry(k).or_default().append(&mut v);
        }
    }
    final_map

    /*for node in root.children(&mut cursor) {
        if T::is_function_node(&node) {
            let name = T::get_node_name(&node, content).unwrap();
            map.entry(format!("{class_name}{name}")).or_default(); // or_default inserts
                                                                   // methods that do not call other methods

            if let Some(body) = T::get_node_body(&node) {
                visit_node_iterative(body, |n| {
                    if T::is_call_expression(&n) {
                        if let Some(called) = T::get_called_function_name(&n, content) {
                            map.entry(format!("{class_name}{name}"))
                                .or_default()
                                .push(format!("{class_name}{called}"));
                        }
                    }
                });
            }
        } else if T::is_class_node(&node) {
            if let (Some(class_name), Some(body)) =
                (T::get_node_name(&node, content), T::get_node_body(&node))
            {
                let child_map = extract_structure::<T>(body, content, format!("{class_name}::"));
                map.extend(child_map);
            }
        }
    }

    map.into_iter()
        .map(|(k, v)| (k, v.into_iter().collect()))
        .collect()*/
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
