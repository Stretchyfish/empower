use empower_engine;
use empower_engine::node_graph::node::node_kind::{NodeKind, NumberNodeState};

fn main()
{
    let mut node_graph = empower_engine::NodeGraph::new();

    let start_node = node_graph.add_node(&NodeKind::Start);
    let number_node = node_graph.add_node(&NodeKind::Number( NumberNodeState::new()));
    let number_node_2 = node_graph.add_node(&NodeKind::Number( NumberNodeState::new() ));

    let bool_node = node_graph.add_node(&NodeKind::Bool);
    let text_node = node_graph.add_node(&NodeKind::Text);
    
    let print_node = node_graph.add_node(&NodeKind::Print);
    let print_node_2 = node_graph.add_node(&NodeKind::Print);
    let print_node_3 = node_graph.add_node(&NodeKind::Print);
    let addition_node = node_graph.add_node(&NodeKind::Addition);
    let multiply_node = node_graph.add_node(&NodeKind::Multiply);

    let number_node_connection_result = node_graph.add_connection(number_node.output_port_keys[0], number_node_2.input_port_keys[0]);
    let print_connection_result = node_graph.add_connection(start_node.output_port_keys[0], print_node.input_port_keys[0]);

    let print_2_result = node_graph.add_connection(bool_node.output_port_keys[0], print_node_2.input_port_keys[1]);

    let a = node_graph.add_connection(start_node.output_port_keys[0], print_node_3.input_port_keys[0]);
    let b = node_graph.add_connection(text_node.output_port_keys[0], print_node_3.input_port_keys[1]);

    match a
    {
        Ok(()) => println!("Added succesffully"),
        Err(e) => println!("{}", e ),

    }

    match b
    {
        Ok(()) => println!("Added succesffully"),
        Err(e) => println!("{}", e ),

    }

    let c = node_graph.add_connection(number_node.output_port_keys[0], addition_node.input_port_keys[0]);

    match c
    {
        Ok(()) => println!("Added succesffully"),
        Err(e) => println!("{}", e ),

    }

    let d = node_graph.add_connection(number_node.output_port_keys[0], multiply_node.input_port_keys[0]);

    match d
    {
        Ok(()) => println!("Added succesffully"),
        Err(e) => println!("{}", e ),

    }

    match print_2_result
    {
        Ok(()) => println!("Added succesffully"),
        Err(e) => println!("{}", e ),
    }

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

    empower_engine::analyser::graph_overview::node_graph_quick_overview(&node_graph);

    // node_graph.execute_node_graph();

    node_graph.start_node_graph();

    while node_graph.is_running()
    {
        node_graph.view_node_graph();
    }
}