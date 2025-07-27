use tree_sitter::Node;

pub trait LanguageSpec {
    fn is_function_node(node: &Node) -> bool;
    fn is_class_node(node: &Node) -> bool;
    fn get_node_name<'a>(node: &Node, content: &'a str) -> Option<&'a str> {
        node.child_by_field_name("name")
            .map(|n| &content[n.byte_range()])
    }
    fn get_node_body<'a>(node: &'a Node<'a>) -> Option<Node<'a>> {
        node.child_by_field_name("body")
    }
    fn is_call_expression(node: &Node) -> bool;
    fn get_called_function_name<'a>(call_node: &Node, content: &'a str) -> Option<&'a str>;
}
