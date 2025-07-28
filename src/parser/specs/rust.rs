use crate::parser::specs::language_spec::LanguageSpec;
use tree_sitter::Node;
pub struct RustSpec;

impl LanguageSpec for RustSpec {
    fn file_type<'a>() -> &'a str {
        ".rs"
    }

    fn is_function_node(node: &Node) -> bool {
        node.kind() == "function_item"
    }

    fn is_class_node(_node: &Node) -> bool {
        false
    }

    fn is_call_expression(node: &Node) -> bool {
        node.kind() == "call_expression"
    }

    fn get_called_function_name<'a>(call_node: &Node, content: &'a str) -> Option<&'a str> {
        call_node
            .child_by_field_name("function")
            .map(|n| &content[n.byte_range()])
    }
}
