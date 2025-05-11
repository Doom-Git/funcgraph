mod input;
mod parse;
mod window;

fn main() {
    let (lang, dir) = input::parse_input();
    let map = parse::create_tree(lang, dir);
    window::draw(map);
}