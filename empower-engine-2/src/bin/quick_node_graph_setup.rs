use empower_engine_2;

fn main()
{
    let mut node_graph = empower_engine_2::NodeGraph::new();

    let start_node = node_graph.add_node(empower_engine_2::NodeType::Start);
    let number_node = node_graph.add_node(empower_engine_2::NodeType::Number);

    empower_engine_2::analyser::graph_overview::node_graph_quick_overview(&node_graph);
}