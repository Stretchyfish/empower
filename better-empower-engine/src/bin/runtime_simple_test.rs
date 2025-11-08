fn main()
{
    let mut node_graph = better_empower_engine::NodeGraph::new();

    let number_node_1 = node_graph.add_node("number");
    let number_node_2 = node_graph.add_node("number");

    let connection_response = node_graph.add_connection(number_node_1.output_port_keys[0], number_node_2.input_port_keys[0]);

    if connection_response.is_err()
    {
        println!("{}", connection_response.err().unwrap());
    }

    better_empower_engine::analyser::node_graph_quick_overview(&node_graph);


    let mut runtime = better_empower_engine::EmpowerRuntime::new(node_graph);
    runtime.execute();
}