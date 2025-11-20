use better_empower_engine;

fn main()
{
    let mut node_graph = better_empower_engine::NodeGraph::new();

    let _ = node_graph.add_node("start");

    better_empower_engine::analyser::node_graph_quick_overview(&node_graph);
}
