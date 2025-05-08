mod input;
mod parse;

fn main() {
    let (lang, dir) = input::parse_input();
    parse::create_tree(lang, dir);
}