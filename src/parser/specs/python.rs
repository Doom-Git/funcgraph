use tree_sitter::Node;
use super::language_spec::LanguageSpec;

pub struct PythonSpec;

impl LanguageSpec for PythonSpec {
    fn is_function_node(node: &Node) -> bool {
        node.kind() == "function_definition"
    }

    fn get_function_name<'a>(node: &Node, content: &'a str) -> Option<&'a str> {
        node.child_by_field_name("name")
            .map(|name_node| &content[name_node.byte_range()])
    }

    fn get_function_body<'a>(node: &'a Node<'a>) -> Option<Node<'a>> {
        node.child_by_field_name("body")
    }

    fn is_call_expression(node: &Node) -> bool {
        node.kind() == "call"
    }

    fn get_called_function_name<'a>(call_node: &Node, content: &'a str) -> Option<&'a str> {
        call_node.child_by_field_name("function")
            .map(|func_node| &content[func_node.byte_range()])
    }
}