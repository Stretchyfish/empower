use std::collections::HashMap;
use std::collections::VecDeque;

use crate::EmpowerData;
use crate::EmpowerKey;
use crate::Node;
use crate::NodeType;
use crate::InputPort;
use crate::OutputPort;

mod node_creation;
mod compiler;

#[derive(Default)]
pub struct EmpowerEngine
{
    nodes: HashMap<EmpowerKey, Node>,
    input_ports: HashMap<EmpowerKey, InputPort>,
    output_ports: HashMap<EmpowerKey, OutputPort>,
    connections: HashMap<EmpowerKey, Vec<EmpowerKey>>,
}

impl EmpowerEngine
{
    pub fn new() -> Self
    {
        let nodes = HashMap::new();
        let input_ports = HashMap::new();
        let output_ports = HashMap::new();
        let connections = HashMap::new();

        // node_creation::create_start_node(&mut nodes, &mut output_ports);
        
        Self 
        {  
            nodes,
            input_ports,
            output_ports,
            connections,
        }
    }

    pub fn add_node(&mut self, node_type: NodeType) -> EmpowerKey
    {
        let mut new_node_key = 0;
        match node_type
        {
            NodeType::Start =>
            {
                println!("Tried adding another start node, only one start node is allowed!");
            }
            NodeType::Integer =>
            {
                new_node_key = node_creation::create_integer_node(&mut self.nodes, &mut self.input_ports, &mut self.output_ports);
            }
        }

        new_node_key
    }

    pub fn remove_node(&mut self, key: EmpowerKey)
    {
        self.nodes.remove(&key); // @TODO, remove input and output ports aswell
    }

    pub fn get_node_input_port_keys(&mut self, node_key: EmpowerKey) -> Option<Vec<EmpowerKey>>
    {
        if !self.nodes.contains_key(&node_key)
        {
            println!("Tried to get input port of node key not in nodes");
            return None;
        }

        let node = self.nodes.get(&node_key).unwrap();

        return Some( node.input_port_keys.clone() );
    }

    pub fn get_node_output_port_keys(&mut self, node_key: EmpowerKey) -> Option<Vec<EmpowerKey>>
    {
        if !self.nodes.contains_key(&node_key)
        {
            println!("Tried to get output port of node key not in nodes");
            return None;
        }

        let node = self.nodes.get(&node_key).unwrap();

        return Some( node.output_port_keys.clone() );
    }

    pub fn get_input_port_value(&mut self, input_port_key: EmpowerKey)
    {
        if !self.input_ports.contains_key(&input_port_key)
        {
            println!("Get input port value tried to access a non-existend input port");
            return;
        }

        let input_port = self.input_ports.get(&input_port_key).unwrap();
        
        println!("Input port ({}) value: {}", input_port_key, input_port.value);
    }

    pub fn get_output_port_value(&mut self, output_port_key: EmpowerKey)
    {
        if !self.output_ports.contains_key(&output_port_key)
        {
            println!("Get output port value tried to access a non-existend input port");
            return;
        }

        let output_port = self.output_ports.get(&output_port_key).unwrap();
        
        println!("Output port ({}) value: {}", output_port_key, output_port.value);
    }

    pub fn set_input_port_value(&mut self, input_port_key: EmpowerKey, value: EmpowerData)
    {
        if !self.input_ports.contains_key(&input_port_key)
        {
            println!("Set input port value tried to access a non-existend input port");
            return;
        }

        let input_port = self.input_ports.get_mut(&input_port_key).unwrap();
        input_port.value = value;
    }


    pub fn add_connection(&mut self, input_port_key: EmpowerKey, output_port_key: EmpowerKey)
    {
        if self.connections.contains_key(&output_port_key)
        {
            let existing_connection = self.connections.get_mut(&output_port_key).unwrap();

            if existing_connection.contains(&input_port_key)
            {
                println!("Could not add connection, as it already exists");
                return;
            }

            existing_connection.push(input_port_key);
            return;
        }

        self.connections.insert(output_port_key, Vec::from([input_port_key]));
    }

    pub fn compile(&mut self)
    {
        // Remove this check later after adding start node behavior
        if self.nodes.is_empty()
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
            let node_to_compile_type = self.nodes.get_mut(&node_to_compile_key).unwrap().node_type.clone();

            // let new_nodes_to_compile: Vec<EmpowerKey> = Vec::new();
            match node_to_compile_type
            {
                NodeType::Integer =>
                {
                    compiler::execute_integer_node(node_to_compile_key, &mut self.nodes, &mut self.input_ports, &mut self.output_ports);
                },

                _ =>
                {
                    println!("Asked to compile a not supported node type");
                }
            }


            next_node_to_compile_index = node_keys_to_compile_queue.len(); // @TODO, change the node_key_queue to not grow infinitely

            let new_nodes_to_compile = self.transfer_connected_port_values(node_to_compile_key);
            node_keys_to_compile_queue.extend(new_nodes_to_compile);

        }
       
        println!("Compiled nodes");
    }

    pub fn debug_compile(&mut self)
    {
        println!("Begin compiling nodes in debug node");

        // Remove this check later after adding start node behavior
        if self.nodes.is_empty()
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
            let node_to_compile_type = self.nodes.get_mut(&node_to_compile_key).unwrap().node_type.clone();

            match node_to_compile_type
            {
                NodeType::Integer =>
                {
                    compiler::execute_debug_integer_node(node_to_compile_key, &mut self.nodes, &mut self.input_ports, &mut self.output_ports);
                },

                _ =>
                {
                    println!("Asked to compile a not supported node type");
                }
            }

            println!(" --> ");

            let new_nodes_to_compile = self.transfer_connected_port_values(node_to_compile_key);
            node_keys_to_compile_queue.extend(new_nodes_to_compile);
            node_keys_to_compile_queue.pop_front();

            compiled_nodes_counter += 1;
        }
       
        println!("Compiled {} nodes", compiled_nodes_counter);
        println!("Finished compiling nodes in debug mode");
    }

    fn transfer_connected_port_values(&mut self, node_key: EmpowerKey) -> Vec<EmpowerKey>
    {
        let mut new_nodes_to_compile = Vec::new();
        
        if !self.nodes.contains_key(&node_key)
        {
            println!("Requested non-existing node in transfer connected port values");
            return new_nodes_to_compile;
        }

        let node = self.nodes.get(&node_key).unwrap();
        let node_output_ports = node.output_port_keys.clone();

        for output_port_key in node_output_ports.iter()
        {
            if !self.output_ports.contains_key(output_port_key)
            {
                continue;
            }

            let output_port = self.output_ports.get(output_port_key).unwrap();
            
            if !self.connections.contains_key(&output_port_key)
            {
                continue;
            }

            let connected_input_ports = self.connections.get(&output_port_key).unwrap();

            for connected_port_key in connected_input_ports.iter()
            {
                if !self.input_ports.contains_key(&connected_port_key)
                {
                    println!("Node Tried to Access an input port not in connected ports");
                    continue;
                }

                let connected_port = self.input_ports.get_mut(connected_port_key).unwrap();
                connected_port.value = output_port.value;
                new_nodes_to_compile.push(connected_port_key.clone());
            }
        }
        return new_nodes_to_compile;
    }
}
