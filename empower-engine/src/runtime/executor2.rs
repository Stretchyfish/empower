use std::collections::{HashMap, HashSet, VecDeque};

use crate::{NodeGraph, NodeGraphKey, PortValue, node_graph::{self, node::{Node, node_kind::NodeResponse, port::port_value}}, utility::log_buffer::LogBuffer};

const START_NODE_KEY: NodeGraphKey = 1;

pub struct EmpowerExecutor
{
    nodes_to_setup: VecDeque<NodeGraphKey>,
    nodes_to_update: HashSet<NodeGraphKey>,
    nodes_to_show: HashSet<NodeGraphKey>,

    cached_output_ports: HashMap<NodeGraphKey, PortValue>,

    debugger: Option<EmpowerDebugger>,
}

impl EmpowerExecutor
{
    pub fn new(running_in_editor: bool) -> Self
    {
        let mut debugger = None;
        if running_in_editor
        {
            debugger = Some( EmpowerDebugger::new() );
        }
        
        EmpowerExecutor
        {
            nodes_to_setup: VecDeque::new(),
            nodes_to_update: HashSet::new(),
            nodes_to_show: HashSet::new(),

            cached_output_ports: HashMap::new(),
            debugger,
        }
    }

    pub fn start(&mut self) -> bool
    {
        self.start_node_graph_from_entry(&START_NODE_KEY)
    }
    
    pub fn start_node_graph_from_entry(&mut self, node_key: &NodeGraphKey) -> bool
    {
        self.nodes_to_setup.push_back( *node_key );

        true
    }

    pub fn is_running(&self) -> bool
    {
        !self.nodes_to_setup.is_empty() && !self.nodes_to_update.is_empty() && !self.nodes_to_show.is_empty()
    }

    pub fn execute(&mut self, node_graph: &mut NodeGraph, ctx: Option<&egui::Context>)
    {
        self.update_nodes(node_graph);
        self.setup_next_node(node_graph);

        if ctx.is_none()
        {
            return;
        }

        self.show_nodes(node_graph, ctx.unwrap());
    }

    fn setup_next_node(&mut self, node_graph: &mut NodeGraph)
    {
        if self.nodes_to_setup.is_empty()
        {
            return;
        }
        
        let node_key = self.nodes_to_setup.pop_front().unwrap();

        let node_response = {
            
            let node = node_graph.nodes.get_mut(&node_key).unwrap();

            // This is done like that because of borrow issues
            // @TODO, find a better way to write this
            let mut inputs = Vec::with_capacity(node.input_port_keys.len());
            for input_port_key in &node.input_port_keys
            {
                let connected_output_port_key = node_graph.connections_in.get(input_port_key);

                if connected_output_port_key.is_none()
                {
                    let input_port = node_graph.input_ports.get(input_port_key).unwrap();
                    inputs.push(&input_port.value);
                    continue;
                }
            
                let connected_output_port_value = self.cached_output_ports.get(&connected_output_port_key.unwrap()).unwrap();
                inputs.push(connected_output_port_value);
            } 

            node.kind.setup(inputs)
        };

        match node_response
        {
            NodeResponse::Continue => self.nodes_to_setup.push_back( node_key ),
            NodeResponse::Finished(port_values) =>
            {
                self.cache_output_ports(&node_graph, &node_key, port_values);
                self.get_next_nodes_to_setup_from_selected_node(&node_graph, &node_key);
            },
            NodeResponse::FinishedWithLog(port_values, log) =>
            {
                self.cache_output_ports(&node_graph, &node_key, port_values);
                self.get_next_nodes_to_setup_from_selected_node(&node_graph, &node_key);
                // self.logs.add_log_info(log);
            },
            NodeResponse::CreateLoop(port_values) => todo!(),
            NodeResponse::ContinueLoop(port_values) => todo!(), // @TODO, consider renaming this to "trigger loop"
            NodeResponse::RestartLoop => todo!(),
            NodeResponse::StopLoop => todo!(),
            NodeResponse::CreateWindow => todo!(),
            NodeResponse::Error(_) => todo!(),
        }
    }

    fn update_nodes(&mut self, node_graph: &mut NodeGraph)
    {
        
    }

    fn show_nodes(&mut self, node_graph: &mut NodeGraph, ctx: &egui::Context)
    {
        
    }

    fn cache_output_ports(&mut self, node_graph: &NodeGraph, node_key: &NodeGraphKey, outputs: Vec<PortValue>)
    {
        let node_handle = node_graph.get_node_handle(node_key);
        
        // @TODO, small optimization possible here is to pre-allocate space in the hashamp

        self.cached_output_ports.extend(
            node_handle.output_port_keys.iter().copied().zip( outputs.into_iter() )
        );
    }

    fn get_next_nodes_to_setup_from_selected_node(&self, node_graph: &NodeGraph, node_key: &NodeGraphKey) -> Vec<NodeGraphKey> 
    {
        let mut connected_nodes_to_setup = Vec::new();

        let node_handle = node_graph.get_node_handle(node_key);

        if node_handle.output_port_keys.is_empty()
        {
            return Vec::new();
        }

        for output_port_key in &node_handle.output_port_keys
        {
            let connected_input_ports = match node_graph.connections_out.get(output_port_key) // These ports can have a 1:N relationship
            {
                Some( connections ) => connections,
                None => continue,
            };

            let output_port_value = self.cached_output_ports.get(output_port_key).unwrap();

            if *output_port_value != PortValue::Trigger(true)
            {
                continue;
            }

            for next_nodes_input_port_key in connected_input_ports
            {
                let input_port = node_graph.input_ports.get(next_nodes_input_port_key).unwrap();
                connected_nodes_to_setup.push( input_port.node_key );
            }
        }

        connected_nodes_to_setup
    }
}

enum NodeSetupResponse
{
    Finished( Vec<PortValue> ),
    AddToUpdate,
    CreateLoop,
    CreateWindow,
}

enum NodeUpdateResponse
{
    Continue,
    Finished( Vec<PortValue> ),
    TriggerLoop,
    StopLoop,
    StopWindow,
}

struct EmpowerDebugger
{
    
}

impl EmpowerDebugger
{
    pub fn new() -> Self
    {
        Self
        {
            
        }
    }
}

