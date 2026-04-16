use std::env;
use empower_engine::{NodeGraph, EmpowerRuntime};

fn main()
{
    let current_directory = env::current_exe().unwrap().parent().unwrap().to_path_buf();
    
    let node_graph = NodeGraph::load( current_directory.join("graph.json"));

    let mut runtime = EmpowerRuntime::new(node_graph, true);
    runtime.execute();
}
