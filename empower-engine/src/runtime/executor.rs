use std::collections::HashSet;
use std::collections::VecDeque;

use crate::NodeGraph;
use crate::NodeGraphKey;
use crate::PortValue;
use crate::node_graph::node::node_kind::NodeSetupResponse;
use crate::node_graph::node::node_kind::NodeUpdateResponse;

use crate::utility::text_buffer::TextBuffer;
use super::analysis;

pub mod window_manager;
pub use window_manager::WindowManager;

pub mod loop_manager;
pub use loop_manager::LoopManager;

#[derive(Clone)]
pub struct EmpowerExecutor
{
    pub node_graph: NodeGraph,
    pub debug_mode: bool,
    pub running_in_editor: bool,

    pub execution_queue: VecDeque<NodeGraphKey>,
    pub nodes_to_update: HashSet<NodeGraphKey>, // @TODO, come up with a better name
    pub window_manager: WindowManager,
    pub loop_manager: LoopManager,

    pub log: TextBuffer,
    pub cached_output_ports: HashSet<NodeGraphKey>,
    pub history: Vec<NodeGraphKey>,
    pub last_executed_node: NodeGraphKey,
}

impl EmpowerExecutor
{
    pub fn new(node_graph: NodeGraph, running_in_editor: bool, debug_mode: bool) -> Self
    {
        let mut window_manager = WindowManager::new();
        window_manager.main_window_key = Some( 0 );

        Self
        {
            node_graph,
            debug_mode,
            running_in_editor, 

            execution_queue: VecDeque::new(),
            nodes_to_update: HashSet::new(),

            window_manager,
            loop_manager: LoopManager::new(),

            log: TextBuffer::new(),
            cached_output_ports: HashSet::new(),
            history: Vec::new(),
            last_executed_node: 0,
        }
    }

    pub fn start_node_graph(&mut self) 
    {
        if self.node_graph.node_count() == 0 { return; }

        let start_node_key: NodeGraphKey = 1; // @TODO, find a better approach
        self.start_node_graph_from_entry(&start_node_key);
    }

    pub fn start_node_graph_from_entry(&mut self, node_key: &NodeGraphKey)
    {
        if self.debug_mode
        {
            analysis::start_debugging(self);
        }

        if *node_key == 1 // Start node key (This check is only for debugging of node graph without a start node, should get removed later)
        {
            let start_node = self.node_graph.nodes.get(node_key).unwrap();

            if start_node.kind.name() != "start"
            {
                panic!("Node Graph is missing start node, will not execute");
            }
        }

        // Use stop node graph to clear any potential cache
        self.stop_node_graph();
        
        // This starts the node graph
        self.execution_queue.push_front(*node_key);
    }

    pub fn stop_node_graph(&mut self)
    {
        self.cached_output_ports.clear();
        self.execution_queue.clear();
        self.nodes_to_update.clear();
        self.window_manager.clear_windows();
    }

    pub fn is_running(&self) -> bool
    {
       !self.execution_queue.is_empty() || !self.nodes_to_update.is_empty()
    }

    pub fn execute_node_graph(&mut self, ui: Option<&mut egui::Ui>)
    {
        if !self.is_running()
        {
            return;
        }
        
        self.setup_nodes();
        self.update_nodes();

        if ui.is_none()
        {
            return;
        }

        self.show_windows(ui.unwrap());
    }

    pub fn setup_nodes(&mut self)
    {
        let next_node_to_setup = self.execution_queue.pop_front();

        if next_node_to_setup.is_none()
        {
            return;
        }

        let next_node_to_setup_key = next_node_to_setup.unwrap();
        
        let node = self.node_graph.nodes.get_mut(&next_node_to_setup_key).unwrap();

        // This is done like that because of borrow issues
        // @TODO, find a better way to write this
        let mut input_port_values = Vec::with_capacity(node.input_port_keys.len());
        for input_port_key in &node.input_port_keys
        {
            let input_port = self.node_graph.input_ports.get(input_port_key).unwrap();
            input_port_values.push(&input_port.value);
        } 

        let setup_response = node.kind.setup(input_port_values);

        match setup_response
        {
            NodeSetupResponse::Finished(outputs) => self.process_node_outputs(&next_node_to_setup_key, outputs),
            NodeSetupResponse::FinishedWithLog(outputs, text_buffer) =>
            {
                self.process_node_outputs(&next_node_to_setup_key, outputs);                
                self.log.add_line(&text_buffer);
            },
            NodeSetupResponse::CreateWindow =>
            {
                self.window_manager.create_window(&next_node_to_setup_key, node.kind.name());
                self.nodes_to_update.insert(next_node_to_setup_key);
            },
            NodeSetupResponse::CreateLoop => todo!(),
            NodeSetupResponse::Error(_) => todo!(),
        }
    }

    pub fn update_nodes(&mut self)
    {
        for node_key in &self.nodes_to_update.clone() // @TODO, find a way to remove this clone
        {
            let node = self.node_graph.nodes.get_mut(&node_key).unwrap();
            let update_response = node.kind.update();

            match update_response
            {
                NodeUpdateResponse::Running => continue,
                NodeUpdateResponse::Finished(outputs) => self.process_node_outputs(node_key, outputs),
            };

            self.nodes_to_update.remove(node_key); // @TODO, consider moving this out of the update loop, and remove after passing all nodes
        }
    }

    fn process_node_outputs(&mut self, node_key: &NodeGraphKey, outputs: Vec<PortValue>)
    {
        let node_handle = self.node_graph.get_node_handle(node_key);

        // Set output values
        self.cached_output_ports.extend(node_handle.output_port_keys);
        self.node_graph.set_output_port_values(&node_key, &outputs);

        // Distribute outputs and determine which nodes to run next
        let new_nodes_to_execute = self.node_graph.distribute_outputs(&node_key); 

        let mut extra_nodes_needed_for_execution = Vec::new();
        for new_node_to_execute_key in &new_nodes_to_execute
        {
            analysis::determine_is_node_is_ready_for_exeuction(*new_node_to_execute_key, &self.node_graph, &mut extra_nodes_needed_for_execution, &mut self.cached_output_ports);
        }

        self.execution_queue.extend(extra_nodes_needed_for_execution);
        self.execution_queue.extend(new_nodes_to_execute);
    }

    pub fn show_windows(&mut self, ui: &mut egui::Ui)
    {
        for (node_key_to_show, window_title) in self.window_manager.get_windows()
        {
            let node = self.node_graph.nodes.get_mut(&node_key_to_show).unwrap();

            if self.window_manager.main_window_key.is_none()
            {
                return; // Should only happen on the first update loop
            }

            let main_window_id = self.window_manager.main_window_key.unwrap();

            if !self.running_in_editor && node_key_to_show == main_window_id // @TODO, find a better way to organize these
            {
                node.kind.show(ui);
                continue;
            }

            let mut window_open = true;
            egui::Window::new(window_title)
            .open(&mut window_open)
            .show(ui.ctx(), |window_ui|
            {
                node.kind.show(window_ui);
            });

            if window_open == true
            {
                continue;
            }

            self.window_manager.remove_window(&node_key_to_show);
            self.nodes_to_update.remove(&node_key_to_show);
        }
    }
}
