use empower_engine::node_graph::{NodeGraph, node::node_kind::PrintNode};

fn main()
{
    let mut new_node_graph = NodeGraph::new("test graph");
    new_node_graph.add_node(PrintNode, None);
}
