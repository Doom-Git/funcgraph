use tree_sitter::Node;
use crate::parser::specs::language_spec::LanguageSpec;
pub struct RustSpec;

impl LanguageSpec for RustSpec {
    fn is_function_node(node: &Node) -> bool {
        node.kind() == "function_item"
    }

    fn get_function_name<'a>(node: &Node, content: &'a str) -> Option<&'a str> {
        node.child_by_field_name("name")
            .map(|n| &content[n.byte_range()])
    }

    fn get_function_body<'a>(node: &'a Node<'a>) -> Option<Node<'a>> {
        node.child_by_field_name("body")
    }

    fn is_call_expression(node: &Node) -> bool {
        node.kind() == "call_expression"
    }

    fn get_called_function_name<'a>(call_node: &Node, content: &'a str) -> Option<&'a str> {
        call_node.child_by_field_name("function")
            .map(|n| &content[n.byte_range()])
    }
}
