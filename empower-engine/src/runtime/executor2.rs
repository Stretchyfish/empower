use std::collections::{HashMap, HashSet, VecDeque};

use crate::{NodeGraph, NodeGraphKey, PortValue, node_graph::{self, node::{Node, node_kind::NodeResponse, port::port_value}}};
use super::EmpowerDebugger;

const START_NODE_KEY: NodeGraphKey = 1;

pub struct EmpowerExecutor
{
    task_queue: VecDeque<Task>,

    cached_output_ports: HashMap<NodeGraphKey, PortValue>,

}

impl EmpowerExecutor
{
    pub fn new(running_in_editor: bool) -> Self
    {
        EmpowerExecutor
        {
            task_queue: VecDeque::new(),

            cached_output_ports: HashMap::new(),
        }
    }

    pub fn start(&mut self) -> bool
    {
        self.start_node_graph_from_entry(&START_NODE_KEY)
    }
    
    pub fn start_node_graph_from_entry(&mut self, node_key: &NodeGraphKey) -> bool
    {
        self.task_queue.push_back( Task::new(*node_key, Action::Setup) );

        true
    }

    pub fn is_running(&self) -> bool
    {
        !self.task_queue.is_empty()
    }

    pub fn execute(&mut self, node_graph: &mut NodeGraph) -> bool
    {
        let next_task = self.task_queue.pop_front();

        if next_task.is_none()
        {
            return false;
        }

        let next_task = next_task.unwrap();

        let node_key = next_task.node_key;

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

            match next_task.action
            {
                Action::Setup => node.kind.setup(inputs),
                Action::Update => todo!(),
            }
        };

        match node_response
        {
            NodeResponse::Continue => self.task_queue.push_back( Task::new(next_task.node_key, Action::Update) ),
            NodeResponse::Finished(port_values) =>
            {
                self.cache_output_ports(&node_graph, &node_key, port_values);
                self.get_next_tasks_from_node(&node_graph, &node_key);
            },
            NodeResponse::CreateLoop(port_values) => todo!(),
            NodeResponse::ContinueLoop(port_values) => todo!(), // @TODO, consider renaming this to "trigger loop"
            NodeResponse::RestartLoop => todo!(),
            NodeResponse::StopLoop => todo!(),
            NodeResponse::CreateWindow => todo!(),
            NodeResponse::Error(_) => todo!(),
        }
        
        true
    }

    fn setup_next_node(&mut self, node_graph: &mut NodeGraph)
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

    fn get_next_tasks_from_node(&self, node_graph: &NodeGraph, node_key: &NodeGraphKey) -> Vec<Task> 
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
                connected_nodes_to_setup.push( Task::new(input_port.node_key, Action::Setup) );
            }
        }

        connected_nodes_to_setup
    }
}

struct Task
{
    node_key: NodeGraphKey,
    action: Action 
}

impl Task
{
    pub fn new(node_key: NodeGraphKey, action: Action) -> Self
    {
        Task
        {
            node_key,
            action
        }
    }
}

enum Action
{
    Setup,
    Update
}
