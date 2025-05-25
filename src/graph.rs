use graphviz_rust::{
    cmd::{CommandArg, Format},
    exec, parse,
    printer::PrinterContext,
};
use std::collections::HashMap;
use std::fmt::Write;

pub fn gen_graph(val: HashMap<String, Vec<String>>) {
    let mut graph_dot = String::with_capacity(1024);
    graph_dot.push_str("digraph functions {\n");

    let mut id_map: HashMap<String, i32> = HashMap::new();
    let mut id_counter = 1;

    for key in val.keys() {
        id_map.insert(key.clone(), id_counter);
        writeln!(graph_dot, "    n{} [label=\"{}\"];", id_counter, key).unwrap();
        id_counter += 1;
    }

    for (caller, callees) in &val {
        if let Some(&caller_id) = id_map.get(caller) {
            for callee in callees {
                if let Some(&callee_id) = id_map.get(callee) {
                    writeln!(graph_dot, "    n{} -> n{};", caller_id, callee_id).unwrap();
                }
            }
        }
    }

    graph_dot.push_str("}\n");
    print_graph(graph_dot);
}

/// Creates an image of the graph
fn print_graph(dot: String) {
    let g = parse(&dot).expect("Failed to parse dot string");
    let mut ctx = PrinterContext::default();
    exec(
        g,
        &mut ctx,
        vec![
            CommandArg::Format(Format::Png),
            CommandArg::Output("graph.png".to_string()),
        ],
    )
    .expect("Failed to generate graph");
}
