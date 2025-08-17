use core::num;

use empower_engine_2;

fn main()
{
    let mut node_graph = empower_engine_2::NodeGraph::new();

    let start_node = node_graph.add_node(empower_engine_2::NodeType::Start);
    let number_node = node_graph.add_node(empower_engine_2::NodeType::Number);
    let number_node_2 = node_graph.add_node(empower_engine_2::NodeType::Number);

    let print_node = node_graph.add_node(empower_engine_2::NodeType::Print);

    let number_node_connection_result = node_graph.add_connection(number_node.output_port_keys[0], number_node_2.input_port_keys[0]);
    let print_connection_result = node_graph.add_connection(start_node.output_port_keys[0], print_node.input_port_keys[0]);

    match number_node_connection_result 
    {
        Ok(()) => println!("Added succesffully"),
        Err(e) => println!("{}", e ),
    }

    match print_connection_result 
    {
        Ok(()) => println!("Added succesffully"),
        Err(e) => println!("{}", e ),
    }

    empower_engine_2::analyser::graph_overview::node_graph_quick_overview(&node_graph);
}