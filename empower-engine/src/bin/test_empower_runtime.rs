use empower_engine::{Empower, NodeGraph};

fn main()
{
    let node_graph = NodeGraph::new();

    let mut empower = Empower::new();
    empower.execute(node_graph);
}