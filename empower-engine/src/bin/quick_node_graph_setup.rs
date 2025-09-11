use empower_engine;

fn main()
{
    let mut node_graph = empower_engine::NodeGraph::new();

    let start_node = node_graph.add_node("start");
    let number_node = node_graph.add_node("number");
    let number_node_2 = node_graph.add_node("number");

    let bool_node = node_graph.add_node("bool");
    let text_node = node_graph.add_node("text");
    
    let print_node = node_graph.add_node("print");
    let print_node_2 = node_graph.add_node("print");
    let print_node_3 = node_graph.add_node("print");
    let addition_node = node_graph.add_node("addition");
    let multiply_node = node_graph.add_node("multiply");

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

    node_graph.execute_node_graph();

}