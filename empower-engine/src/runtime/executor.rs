use std::collections::HashSet;

use crate::NodeGraph;
use crate::NodeGraphKey;
use crate::PortValue;
use crate::node_graph::node::node_kind::NodeSetupResponse;
use crate::node_graph::node::node_kind::NodeUpdateResponse;

use crate::runtime::executor::task_manager::TaskId;
use crate::utility::log_buffer::LogBuffer;
use crate::utility::text_buffer::TextBuffer;
use super::analysis;

use chrono::DateTime;
use chrono::Local;

pub mod task_manager;
pub use task_manager::TaskManager;

mod task;

#[derive(Clone)]
pub struct EmpowerExecutor
{
    pub debug_mode: Option<DebugMode>,
    pub running_in_editor: bool,

    pub task_manager: TaskManager,

    pub logs: LogBuffer,
    pub cached_output_ports: HashSet<NodeGraphKey>,
    pub history: TextBuffer, // @TODO, Multiple values here should only be saved in debug mode
    pub start_time: DateTime<Local>,
    pub last_executed_node: NodeGraphKey,
}

impl EmpowerExecutor
{
    pub fn new(running_in_editor: bool, debug_mode: bool) -> Self
    {
        Self
        {
            debug_mode: None,
            running_in_editor, 

            task_manager: TaskManager::new(),

            logs: LogBuffer::new(1000),
            cached_output_ports: HashSet::new(),
            history: TextBuffer::new(),
            start_time: Local::now(),
            last_executed_node: 0,
        }
    }

    pub fn start_node_graph(&mut self, node_graph: &mut NodeGraph) 
    {
        if node_graph.node_count() == 0 { return; }

        let start_node_key: NodeGraphKey = 1; // @TODO, find a better approach
        self.start_node_graph_from_entry(node_graph, &start_node_key);
    }

    pub fn start_node_graph_from_entry(&mut self, node_graph: &mut NodeGraph, node_key: &NodeGraphKey)
    {
        if self.debug_mode.is_some()
        {
            // analysis::start_debugging(self);
        }

        if *node_key == 1 // Start node key (This check is only for debugging of node graph without a start node, should get removed later)
        {
            let start_node = node_graph.nodes.get(node_key).unwrap();

            if start_node.kind.name() != "start"
            {
                panic!("Node Graph is missing start node, will not execute");
            }
        }

        // Use stop node graph to clear any potential cache
        // self.stop_node_graph();
        
        // This starts the node graph
        let task_id = self.task_manager.create_task();
        self.task_manager.add_node_key(&task_id, node_key);
        // self.execution_queue.push_front(*node_key);
    }

    pub fn stop_node_graph(&mut self)
    {
        // @TODO, this whole thing needs a re-work
        self.cached_output_ports.clear();
        self.task_manager.tasks.clear();
    }

    pub fn is_running(&self) -> bool
    {
        self.task_manager.has_tasks()
    }

    pub fn execute_node_graph(&mut self, node_graph: &mut NodeGraph, ctx: Option<&egui::Context>)
    {
        if !self.is_running()
        {
            // @TODO, make this return an enum value with the state
            return;
        }

        self.task_manager.cleanup_finished_tasks(&mut self.history); // @TODO, consider combining this with continue_loop due to the code flow

        self.setup_nodes(node_graph);
        self.update_nodes(node_graph);

        if ctx.is_none()
        {
            return;
        }

        self.show_nodes(node_graph, ctx.unwrap());
    }

    pub fn setup_nodes(&mut self, node_graph: &mut NodeGraph)
    {
        let jobs = self.task_manager.get_next_nodes_to_setup(); // @TODO, it is a bit unclear which action has side effect like this or not

        for job in jobs
        {
            let task_id = job.task_id;
            let next_node_to_setup_key = job.node_key;
    
            let node = node_graph.nodes.get_mut(&next_node_to_setup_key).unwrap();

            // This is done like that because of borrow issues
            // @TODO, find a better way to write this
            let mut input_port_values = Vec::with_capacity(node.input_port_keys.len());
            for input_port_key in &node.input_port_keys
            {
                let input_port = node_graph.input_ports.get(input_port_key).unwrap();
                input_port_values.push(&input_port.value);
            } 

            let setup_response = node.kind.setup(input_port_values);

            self.history.add_line(&format!("Setup (task: {}) (node: {})", task_id, next_node_to_setup_key));

            match setup_response
            {
                NodeSetupResponse::Began => // @TODO, come up with better name
                {
                    self.task_manager.add_node_to_update_key(&task_id, &next_node_to_setup_key);
                    self.history.add_line(&format!("Added to update (task: {}) (node: {})", task_id, next_node_to_setup_key));
                },
                NodeSetupResponse::Finished(outputs) => self.process_node_outputs(node_graph, &task_id, &next_node_to_setup_key, outputs),
                NodeSetupResponse::FinishedWithLog(outputs, text_buffer) =>
                {
                    self.process_node_outputs(node_graph, &task_id, &next_node_to_setup_key, outputs); // @TODO, come up with better naming, since this also adds next nodes to setup               
                    self.logs.add_log_info(text_buffer);
                },
                NodeSetupResponse::CreateWindow =>
                {
                    // @TODO, consider if add to update and add to show should be seperated for clarity?
                    self.task_manager.add_node_to_show_key(&task_id, &next_node_to_setup_key, node.kind.name());

                    self.history.add_line(&format!("Added to update and window (task: {}) (node: {})", task_id, next_node_to_setup_key));
                    // self.nodes_to_update.insert(next_node_to_setup_key);
                },
                NodeSetupResponse::CreateLoop(outputs) =>
                {
                    self.task_manager.add_node_to_update_key(&task_id, &next_node_to_setup_key);
                    let loop_task_id = self.task_manager.add_loop(&job);

                    self.history.add_line(&format!("Created task for loop ({})", loop_task_id));
                    self.process_node_outputs(node_graph, &loop_task_id, &next_node_to_setup_key, outputs);
                },
                NodeSetupResponse::RestartLoop =>
                {
                    self.history.add_line(&format!("Restarted loop for task (task: {})", task_id));
                    self.task_manager.stop_task(&task_id);
                },
                NodeSetupResponse::StopLoop =>
                {
                    self.history.add_line(&format!("Stop loop for task (task: {})", task_id));
                    self.task_manager.remove_loop(&task_id);
                },
                NodeSetupResponse::Error(_) => todo!(),
            }
        }
    }

    pub fn update_nodes(&mut self, node_graph: &mut NodeGraph)
    {
        let jobs = self.task_manager.get_next_nodes_to_update();

        for job in jobs
        {
            let task_id = job.task_id;
            let node_to_update_key = job.node_key;
            
            let node = node_graph.nodes.get_mut(&node_to_update_key).unwrap();
            let update_response = node.kind.update();

            match update_response
            {
                NodeUpdateResponse::Running => continue,
                NodeUpdateResponse::Finished(outputs) =>
                {
                    self.process_node_outputs(node_graph, &task_id, &node_to_update_key, outputs);
                    self.task_manager.remove_node_from_update(&task_id, &node_to_update_key);

                    // @TODO, need to add removal from windows
                },
                NodeUpdateResponse::ContinueLoop(outputs) =>
                {
                    let potential_new_loop_task_id = self.task_manager.continue_loop(&task_id, &node_to_update_key);

                    if potential_new_loop_task_id.is_none() // @TODO, this whole approach needs a second look
                    {
                        continue;
                    };

                    self.history.add_line(&format!("Created task for loop ({})", potential_new_loop_task_id.unwrap()));
                    self.process_node_outputs(node_graph, &potential_new_loop_task_id.unwrap(), &node_to_update_key, outputs);                
                },
            };

            // self.nodes_to_update.remove(node_key); // @TODO, consider moving this out of the update loop, and remove after passing all nodes
        }
    }

    pub fn show_nodes(&mut self,node_graph: &mut NodeGraph, ctx: &egui::Context)
    {
        let show_jobs = self.task_manager.get_nodes_to_show();

        for show_job in &show_jobs
        {
            let node = node_graph.nodes.get_mut(&show_job.node_key).unwrap();

            let mut viewport_got_closed = false;

            let title = show_job.window_title.clone();
            ctx.show_viewport_immediate(
                egui::ViewportId::from_hash_of(title.clone()),
                egui::ViewportBuilder::default()
                .with_title(title)
                .with_inner_size([600.0, 400.0]),
                |ctx, _class| {

                    viewport_got_closed = ctx.input(|i| i.viewport().close_requested());

                    egui::CentralPanel::default().show(ctx, |ui| {
                        node.kind.show(ui);
                    });
                },
            );

            if !viewport_got_closed
            {
                continue;
            }
            
            self.task_manager.remove_node_from_update(&show_job.task_id, &show_job.node_key);
            self.task_manager.remove_node_from_show(&show_job.task_id, &show_job.node_key);
        }

    }

    fn process_node_outputs(&mut self, node_graph: &mut NodeGraph, task_id: &TaskId, node_key: &NodeGraphKey, outputs: Vec<PortValue>)
    {
        let node_handle = node_graph.get_node_handle(node_key);

        // Set output values
        self.cached_output_ports.extend(node_handle.output_port_keys);
        node_graph.set_output_port_values(&node_key, &outputs);

        // Distribute outputs and determine which nodes to run next
        let new_nodes_to_execute = node_graph.distribute_outputs(&node_key); 

        let mut extra_nodes_needed_for_execution = Vec::new();
        for new_node_to_execute_key in &new_nodes_to_execute
        {
            analysis::determine_is_node_is_ready_for_exeuction(*new_node_to_execute_key, &node_graph, &mut extra_nodes_needed_for_execution, &mut self.cached_output_ports);
        }

        self.task_manager.add_nodes_to_setup_keys(task_id, &extra_nodes_needed_for_execution);
        self.task_manager.add_nodes_to_setup_keys(task_id, &new_nodes_to_execute);

        self.history.add_line(&format!("Added to setup (task: {}) (nodes: {:?} + {:?})", task_id, extra_nodes_needed_for_execution, new_nodes_to_execute));

        // self.execution_queue.extend(extra_nodes_needed_for_execution);
        // self.execution_queue.extend(new_nodes_to_execute);
    }

    fn process_debug_single_action(&mut self, node_affected: &NodeGraphKey)
    {
        if self.debug_mode.is_none()
        {
            return;
        }


        



        
    
    }
}

#[derive(Clone)]
struct DebugMode
{
    pub delay_between_each_execution: f32,
}
