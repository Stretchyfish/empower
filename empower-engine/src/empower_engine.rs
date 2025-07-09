use std::collections::VecDeque;

use empower_node_graph::node;
use empower_node_graph::EmpowerData;
use empower_node_graph::EmpowerKey;
use empower_node_graph::EmpowerNodeGraph;
use empower_node_graph::NodeType;

mod compiler;
mod empower_result;
use empower_result::EmpowerResult;

// @TODO, add a struct for stepwise debug compile

pub fn debug_compile(node_graph: &mut EmpowerNodeGraph) -> EmpowerResult
{
    let mut empower_result = EmpowerResult::default(); // @TODO, make a new function
    println!("Begin compiling nodes in debug node");

    // Remove this check later after adding start node behavior
    if node_graph.nodes.is_empty()
    {
        println!("Has no start node, will not compile");
        return empower_result; // @TODO, expand empower result with usefull information here
    }

    debug_compile_global_variables(node_graph);

    let mut node_keys_to_compile_queue: VecDeque<EmpowerKey> = VecDeque::new();
    node_keys_to_compile_queue.push_back(0); // this will assume the first node is always the start node

    let mut compiled_nodes_counter = 0;
    
    while !node_keys_to_compile_queue.is_empty() // this is to ensure no crashes
    {
        print!("Compiler queue: ");
        node_keys_to_compile_queue.iter().for_each(|key| print!("{} ,", key));
        print!("\n");

        let node_to_compile_key: EmpowerKey = node_keys_to_compile_queue[0];

        // let new_nodes_to_compile = debug_compile_node(node_graph, node_to_compile_key);
        debug_compile_node(&mut empower_result, node_graph, node_to_compile_key);

        let new_nodes_to_compile = get_connected_trigger_ports(node_graph, &node_to_compile_key);
        

        // if !node_graph.nodes.contains_key(&node_to_compile_key)
        // {
        //     println!("Requested to compile node [{}] but node does not exist", node_to_compile_key);
        //     return;
        // };

        // let node_to_compile_type = node_graph.nodes.get_mut(&node_to_compile_key).unwrap().node_type;

        // match node_to_compile_type
        // {
        //     NodeType::IntegerVariable =>
        //     {
        //         compiler::execute_debug_integer_node(node_to_compile_key, &mut node_graph.nodes, &mut node_graph.input_ports, &mut node_graph.output_ports);
        //     },

        //     NodeType::Addition =>
        //     {
        //         compiler::execute_debug_addition_node(node_to_compile_key, &mut node_graph.nodes, &mut node_graph.input_ports, &mut node_graph.output_ports);
        //     },

        //     _ =>
        //     {
        //         println!("Asked to compile a not supported node type");
        //     }
        // }

        // println!(" --> ");

        // let new_nodes_to_compile = transfer_connected_port_values(node_graph, node_to_compile_key);
        println!("Next nodes to compile: {:?}", new_nodes_to_compile);
        node_keys_to_compile_queue.extend(new_nodes_to_compile);
        node_keys_to_compile_queue.pop_front();

        compiled_nodes_counter += 1;
    }
   
    println!("Compiled {} nodes", compiled_nodes_counter);
    println!("Finished compiling nodes in debug mode");

    empower_result
}

fn debug_compile_node(empower_result: &mut EmpowerResult, node_graph: &mut EmpowerNodeGraph, node_key: EmpowerKey) -> Vec<EmpowerKey>
{
    if !node_graph.nodes.contains_key(&node_key)
    {
        println!("Requested to compile node [{}] but node does not exist", node_key);
        return Vec::new();
    };

    let node_to_compile_type = node_graph.nodes.get_mut(&node_key).unwrap().node_type;

    match node_to_compile_type
    {
        NodeType::Start =>
        {
            println!("Executed start node");
        },

        NodeType::IntegerVariable =>
        {
            compiler::execute_debug_integer_node(node_key, &mut node_graph.nodes, &mut node_graph.input_ports, &mut node_graph.output_ports);
        },

        NodeType::Addition =>
        {
            compiler::execute_debug_addition_node(node_key, &mut node_graph.nodes, &mut node_graph.input_ports, &mut node_graph.output_ports);
        },

        NodeType::Print =>
        {
            compiler::execute_debug_print_node(node_key, &mut node_graph.nodes, &mut node_graph.input_ports, &mut node_graph.output_ports, empower_result);
        },

        _ =>
        {
            println!("Asked to compile a not supported node type {}", node_key);
        }
    }

    println!(" --> ");

    transfer_connected_port_values(node_graph, node_key)
} 

fn debug_compile_global_variables(node_graph: &mut EmpowerNodeGraph)
{
    println!("Compiling global variables");
    let mut global_variables_keys: VecDeque<EmpowerKey> = VecDeque::new();
//    let mut global_variables_keys = Vec::new();

    // Find all rouge math variables
    for (node_key, node) in node_graph.nodes.iter_mut()
    {
        if node.node_type != NodeType::IntegerVariable && node.node_type != NodeType::Addition // @TODO, could also be an addition variable
        {
            continue;
        }

        for input_port_key in node.input_port_keys.iter_mut()
        {
            if !node_graph.connections_in.contains_key(&input_port_key)
            {
                global_variables_keys.push_back(*node_key);
                continue; 
            }
        }
    }

    // println!("Global variables list: {:?}", global_variables_keys);
    
    // Compile connected math nodes
    while !global_variables_keys.is_empty() // this is to ensure no crashes
    {
        let node_key = global_variables_keys[0];
        // debug_compile_node(node_graph, node_key); 

        // @TODO, find a better way to approach EmpowerResult in this function
        let all_connected_nodes = debug_compile_node(&mut EmpowerResult::default(), node_graph, node_key);

        let mut more_nodes_to_compile = Vec::new();

        // @TODO, During compilation of global variables, make a check that it has not been compiled before
        for connected_node_key in all_connected_nodes
        {
           let connected_node = node_graph.nodes.get(&connected_node_key).unwrap();

            match connected_node.node_type 
            {
                NodeType::IntegerVariable =>
                {
                    more_nodes_to_compile.push(connected_node_key);
                },

                NodeType::Addition =>
                {
                    more_nodes_to_compile.push(connected_node_key);
                },

                _ =>
                {

                },
            }
        }

        println!("Added more nodes to compile: {:?}", more_nodes_to_compile);

        global_variables_keys.extend(more_nodes_to_compile);
        global_variables_keys.pop_front();
    }



    // for node_key in global_variables_keys
    // {
    // }

    println!("Finished compiling global variables");
}

fn compile_node(node_graph: &mut EmpowerNodeGraph, node_key: EmpowerKey)
{
    println!("Begin compiling nodes in debug node");

    // Remove this check later after adding start node behavior
    if node_graph.nodes.contains_key(&node_key)
    {
        println!("Node graph does not have the node requested to compile.");
        return;
    }

    let node_to_compile_type = node_graph.nodes.get_mut(&node_key).unwrap().node_type;

    match node_to_compile_type
    {
        NodeType::IntegerVariable =>
        {
            compiler::execute_debug_integer_node(node_key, &mut node_graph.nodes, &mut node_graph.input_ports, &mut node_graph.output_ports);
        },

        _ =>
        {
            println!("Asked to compile a not supported node type");
        }
    }
    transfer_connected_port_values(node_graph, node_key);
    println!("Compiled {} node", node_key);
}

// @TODO, this function should probably be in the node graph? Not the engine
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
            println!("Node has no output ports in value transfer");
            continue;
        }

        let output_port = node_graph.output_ports.get(output_port_key).unwrap();
        
        if !node_graph.connections_out.contains_key(&output_port_key)
        {
            println!("(tmp) Node has no connection in value transfer");
            continue;
        }

        let connected_input_ports = node_graph.connections_out.get(&output_port_key).unwrap();

        for connected_port_key in connected_input_ports.iter()
        {
            if !node_graph.input_ports.contains_key(&connected_port_key)
            {
                println!("Node Tried to Access an input port not in connected ports");
                continue;
            }

            let connected_port = node_graph.input_ports.get_mut(connected_port_key).unwrap();
            connected_port.value = output_port.value;
            new_nodes_to_compile.push(connected_port.node_key);
            // new_nodes_to_compile.push(connected_port_key.clone());
        }
    }
    new_nodes_to_compile
}

fn get_connected_trigger_ports(node_graph: &mut EmpowerNodeGraph, node_key: &EmpowerKey) -> Vec<EmpowerKey>
{
    let node = node_graph.nodes.get(node_key).unwrap();

    if node.output_port_keys.len() == 0
    {
        return Vec::new();
    }

    let trigger_output_port_key = node.output_port_keys[0];

    if !node_graph.connections_out.contains_key(&trigger_output_port_key)
    {
        return Vec::new();
    } 

    let first_output_port = node_graph.output_ports.get(&trigger_output_port_key).unwrap();

    if first_output_port.value != EmpowerData::Trigger
    {
       return Vec::new();
    }

    let trigger_port_connected_ports = node_graph.connections_out.get(&node.output_port_keys[0]).unwrap();

    let mut connected_nodes = Vec::new();
    for connect_port in trigger_port_connected_ports
    {
        let input_port = node_graph.input_ports.get(connect_port).unwrap();
        connected_nodes.push(input_port.node_key);
    }

    // @TODO, find a more effecient way to write this function
    return connected_nodes;
}