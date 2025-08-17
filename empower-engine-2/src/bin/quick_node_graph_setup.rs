use core::num;

use empower_engine_2;

fn main()
{
    let mut node_graph = empower_engine_2::NodeGraph::new();

    let start_node = node_graph.add_node(empower_engine_2::NodeType::Start);
    let number_node = node_graph.add_node(empower_engine_2::NodeType::Number);
    let number_node_2 = node_graph.add_node(empower_engine_2::NodeType::Number);

    let add_result = node_graph.add_connection(number_node.output_port_keys[0], number_node_2.input_port_keys[0]);

    match add_result 
    {
        Ok(()) => println!("Added succesffully"),
        Err(E) => println!("{}", E ),
    }

    empower_engine_2::analyser::graph_overview::node_graph_quick_overview(&node_graph);
}