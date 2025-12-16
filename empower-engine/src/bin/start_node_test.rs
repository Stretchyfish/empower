use empower_engine;

fn main()
{
    let mut node_graph = empower_engine::NodeGraph::new();

    let _ = node_graph.add_node("start");

    empower_engine::analyser::node_graph_quick_overview(&node_graph);
}
