use std::collections::VecDeque;

use empower_node_graph::EmpowerKey;
use empower_node_graph::EmpowerNodeGraph;
use empower_node_graph::NodeType;

mod compiler;

pub fn compile(node_graph: &mut EmpowerNodeGraph)
{
    // Remove this check later after adding start node behavior
    if node_graph.nodes.is_empty()
    {
        println!("Has no start node, will not compile");
        return;
    }

    let mut node_keys_to_compile_queue: Vec<i32> = Vec::new();
    node_keys_to_compile_queue.push(0); // this will assume the first node is always the start node
    let mut next_node_to_compile_index = 0; // This will avoid circulatory behavior
    
    while node_keys_to_compile_queue.len() > next_node_to_compile_index // this is to ensure no crashes
    {
        let node_to_compile_key: EmpowerKey = node_keys_to_compile_queue[next_node_to_compile_index];
        let node_to_compile_type = node_graph.nodes.get_mut(&node_to_compile_key).unwrap().node_type.clone();

        // let new_nodes_to_compile: Vec<EmpowerKey> = Vec::new();
        match node_to_compile_type
        {
            NodeType::IntegerVariable =>
            {
                compiler::execute_integer_node(node_to_compile_key, &mut node_graph.nodes, &mut node_graph.input_ports, &mut node_graph.output_ports);
            },

            _ =>
            {
                println!("Asked to compile a not supported node type");
            }
        }


        next_node_to_compile_index = node_keys_to_compile_queue.len(); // @TODO, change the node_key_queue to not grow infinitely

        let new_nodes_to_compile = transfer_connected_port_values(node_graph,node_to_compile_key);
        node_keys_to_compile_queue.extend(new_nodes_to_compile);

    }
   
    println!("Compiled nodes");
}

pub fn debug_compile(node_graph: &mut EmpowerNodeGraph)
{
    println!("Begin compiling nodes in debug node");

    // Remove this check later after adding start node behavior
    if node_graph.nodes.is_empty()
    {
        println!("Has no start node, will not compile");
        return;
    }

    let mut node_keys_to_compile_queue: VecDeque<EmpowerKey> = VecDeque::new();
    node_keys_to_compile_queue.push_back(0); // this will assume the first node is always the start node

    let mut compiled_nodes_counter = 0;
    
    while !node_keys_to_compile_queue.is_empty() // this is to ensure no crashes
    {
        print!("Compiler queue: ");
        node_keys_to_compile_queue.iter().for_each(|key| print!("{} ,", key));
        print!("\n");

        let node_to_compile_key: EmpowerKey = node_keys_to_compile_queue[0];
        let node_to_compile_type = node_graph.nodes.get_mut(&node_to_compile_key).unwrap().node_type;

        match node_to_compile_type
        {
            NodeType::IntegerVariable =>
            {
                compiler::execute_debug_integer_node(node_to_compile_key, &mut node_graph.nodes, &mut node_graph.input_ports, &mut node_graph.output_ports);
            },

            _ =>
            {
                println!("Asked to compile a not supported node type");
            }
        }

        println!(" --> ");

        let new_nodes_to_compile = transfer_connected_port_values(node_graph, node_to_compile_key);
        node_keys_to_compile_queue.extend(new_nodes_to_compile);
        node_keys_to_compile_queue.pop_front();

        compiled_nodes_counter += 1;
    }
   
    println!("Compiled {} nodes", compiled_nodes_counter);
    println!("Finished compiling nodes in debug mode");
}

fn transfer_connected_port_values(node_graph: &mut EmpowerNodeGraph, node_key: EmpowerKey) -> Vec<EmpowerKey>
{
    let mut new_nodes_to_compile = Vec::new();
    
    if !node_graph.nodes.contains_key(&node_key)
    {
        println!("Requested non-existing node in transfer connected port values");
        return new_nodes_to_compile;
    }

    let node = node_graph.nodes.get(&node_key).unwrap();
    let node_output_ports = node.output_port_keys.clone();

    for output_port_key in node_output_ports.iter()
    {
        if !node_graph.output_ports.contains_key(output_port_key)
        {
            continue;
        }

        let output_port = node_graph.output_ports.get(output_port_key).unwrap();
        
        if !node_graph.connections.contains_key(&output_port_key)
        {
            continue;
        }

        let connected_input_ports = node_graph.connections.get(&output_port_key).unwrap();

        for connected_port_key in connected_input_ports.iter()
        {
            if !node_graph.input_ports.contains_key(&connected_port_key)
            {
                println!("Node Tried to Access an input port not in connected ports");
                continue;
            }

            let connected_port = node_graph.input_ports.get_mut(connected_port_key).unwrap();
            connected_port.value = output_port.value;
            new_nodes_to_compile.push(connected_port_key.clone());
        }
    }
    return new_nodes_to_compile;
}
