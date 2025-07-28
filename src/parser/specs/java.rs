use super::language_spec::LanguageSpec;
use tree_sitter::Node;

pub struct JavaSpec;

impl LanguageSpec for JavaSpec {
    fn file_type<'a>() -> &'a str {
        ".java"
    }

    fn is_function_node(node: &Node) -> bool {
        node.kind() == "method_declaration"
    }

    fn is_class_node(node: &Node) -> bool {
        node.kind() == "class_declaration"
    }

    fn is_call_expression(node: &Node) -> bool {
        node.kind() == "method_invocation"
    }

    fn get_called_function_name<'a>(call_node: &Node, content: &'a str) -> Option<&'a str> {
        call_node
            .child_by_field_name("name")
            .map(|n| &content[n.byte_range()])
    }
}
