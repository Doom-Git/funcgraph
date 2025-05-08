use std::path::PathBuf;
use tree_sitter::{Node, Parser};

pub fn create_tree(lang: String, dir: PathBuf) {
	let mut parser = Parser::new();
	// TODO: implement better language chooser
	parser.set_language(&tree_sitter_rust::LANGUAGE.into()).expect("Error loading Rust grammar!");

	let content = std::fs::read_to_string(dir).expect("File not found!");
	let tree = parser.parse(&content, None);
	println!("{:?}", tree.as_ref().unwrap().root_node().to_sexp());
	extract_actual_structure(tree.unwrap().root_node(), content);
}


fn extract_actual_structure(root: Node, content: String) {
    let mut cursor = root.walk();

    for function in root.children(&mut cursor).filter(|n| n.kind() == "function_item") {
        let name_node = function.child_by_field_name("name").unwrap();
        let body_node = function.child_by_field_name("body").unwrap();
        let function_name = &content[name_node.byte_range()];

        let mut found_call = false;

        visit_node_recursive(body_node, &content, &mut |call_node| {
			if let Some(called_fn_node) = call_node.child_by_field_name("function") {
				let called_fn_name = &content[called_fn_node.byte_range()];
				println!("{function_name} calls {called_fn_name}");
			}
		});
		

        // Du kannst hier auch ein Flag setzen, falls kein Aufruf gefunden wurde
    }
}

fn visit_node_recursive<F>(node: Node, content: &str, on_call: &mut F)
where
    F: FnMut(Node),
{
    if node.kind() == "call_expression" {
        on_call(node);
    }

    for i in 0..node.child_count() {
        if let Some(child) = node.child(i) {
            visit_node_recursive(child, content, on_call);
        }
    }
}