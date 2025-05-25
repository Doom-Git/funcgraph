use tree_sitter::Node;

pub trait LanguageSpec {
    fn is_function_node(node: &Node) -> bool;
    fn get_function_name<'a>(node: &Node, content: &'a str) -> Option<&'a str>;
    fn get_function_body<'a>(node: &'a Node<'a>) -> Option<Node<'a>>;
    fn is_call_expression(node: &Node) -> bool;
    fn get_called_function_name<'a>(call_node: &Node, content: &'a str) -> Option<&'a str>;
}