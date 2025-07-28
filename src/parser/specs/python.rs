use super::language_spec::LanguageSpec;
use tree_sitter::Node;

pub struct PythonSpec;

impl LanguageSpec for PythonSpec {
    fn file_type<'a>() -> &'a str {
        ".py"
    }

    fn is_function_node(node: &Node) -> bool {
        node.kind() == "function_definition"
    }

    fn is_class_node(node: &Node) -> bool {
        node.kind() == "class_definition"
    }

    fn is_call_expression(node: &Node) -> bool {
        node.kind() == "call"
    }

    fn get_called_function_name<'a>(call_node: &Node, content: &'a str) -> Option<&'a str> {
        call_node
            .child_by_field_name("function")
            .map(|func_node| &content[func_node.byte_range()])
    }
}
