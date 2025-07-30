use std::fmt::Write;
use std::process::{Command, Stdio};
use std::{collections::HashMap, io::Write as IO_Write};

pub fn gen_graph(val: HashMap<String, Vec<String>>) -> String {
    let mut graph_dot = String::with_capacity(1024);
    graph_dot.push_str("digraph functions {\n    overlap=false;\n");

    let mut id_map: HashMap<String, i32> = HashMap::new();
    let mut id_counter = 1;

    for key in val.keys() {
        id_map.insert(key.clone(), id_counter);
        writeln!(graph_dot, "    n{id_counter} [label=\"{key}\"];").unwrap();
        id_counter += 1;
    }

    for (caller, callees) in &val {
        if let Some(&caller_id) = id_map.get(caller) {
            for callee in callees {
                if let Some(&callee_id) = id_map.get(callee) {
                    writeln!(graph_dot, "    n{caller_id} -> n{callee_id};").unwrap();
                }
            }
        }
    }

    /*for (caller, callees) in &val {
        // ID für den Caller, falls noch nicht vorhanden
        let caller_id = *id_map.entry(caller.clone()).or_insert_with(|| {
            let id = id_counter;
            writeln!(graph_dot, "    n{} [label=\"{}\"];", id, caller).unwrap();
            id_counter += 1;
            id
        });

        for callee in callees {
            // ID für den Callee, falls noch nicht vorhanden
            let callee_id = *id_map.entry(callee.clone()).or_insert_with(|| {
                let id = id_counter;
                writeln!(graph_dot, "    n{} [label=\"{}\"];", id, callee).unwrap();
                id_counter += 1;
                id
            });

            writeln!(graph_dot, "    n{} -> n{};", caller_id, callee_id).unwrap();
        }
    }*/

    graph_dot.push_str("}\n");
    graph_dot
}

/// Creates an image of the graph
pub fn print_graph(dot: String, engine: String) {
    let mut child = Command::new(engine) // "fdp", "sfdp", "twopi"
        .arg("-Tsvg")
        .arg("-o")
        .arg("graph.svg")
        .stdin(Stdio::piped())
        .spawn()
        .expect("Failed to start Graphviz process");

    {
        let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        stdin
            .write_all(dot.as_bytes())
            .expect("Failed to write DOT input");
    }

    let output = child.wait().expect("Graphviz process failed");
    assert!(output.success(), "Graphviz rendering failed");
}
