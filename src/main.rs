mod input;
mod parse;
mod graph;

fn main() {
    let (lang, dir) = input::parse_input();
    let map = parse::create_tree(lang, dir);
    graph::gen_graph(map);
}