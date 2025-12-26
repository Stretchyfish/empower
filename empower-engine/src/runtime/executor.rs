use std::collections::HashMap;
use std::collections::VecDeque;

use crate::NodeGraph;
use crate::NodeGraphKey;
use crate::PortValue;
use crate::utility::text_buffer::TextBuffer;
use super::analysis;

use crate::node_graph::node::NodeFunction;

#[derive(Clone)]
pub struct EmpowerExecutor
{
    pub node_graph: NodeGraph,
    pub debug_mode: bool,
    // last_executed_node: NodeGraphKey, // @TODO, consider doing this as a clone, to avoid changed persisting between execution in studio
    // executing_new_node: bool, // @TODO, consider if this is the best way of triggering the setup functions
    pub execution_queue: VecDeque<NodeGraphKey>,
    pub window_execution: Vec<NodeGraphKey>, // @Find better names for these
    pub background_execution: Vec<NodeGraphKey>,
    pub window_manager: WindowManager, // @TODO, keeping this public for debuggging purposes
    pub running_in_editor: bool,
    pub log: TextBuffer,
    // log: TextBuffer,
}

impl EmpowerExecutor
{
    pub fn new(node_graph: NodeGraph, running_in_editor: bool, debug_mode: bool) -> Self
    {
        let mut window_manager = WindowManager::new();
        window_manager.main_window = Some( 0 );

        Self
        {
            node_graph,
            debug_mode,
            // last_executed_node: 0, // @TODO, find a better approach, its currently set to 0, because 0 is unsued
            // executing_new_node: true,
            execution_queue: VecDeque::new(),
            window_execution: Vec::new(),
            background_execution: Vec::new(),
            window_manager,
            running_in_editor, 
            log: TextBuffer::new(),
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

        self.execution_queue = VecDeque::from( analysis::detect_execution_order_2(&mut self.node_graph) );

        // self.execution_queue.clear();

        // let rouge_nodes = analysis::detect_rouge_nodes(&self.node_graph);

        // self.execution_queue.extend(rouge_nodes);
        // self.execution_queue.push_back(*node_key);
    }

    pub fn stop_node_graph(&mut self)
    {
        self.execution_queue.clear();
    }

    pub fn is_running(&self) -> bool
    {
       !self.execution_queue.is_empty() || !self.background_execution.is_empty()
    }

    pub fn execute_node_graph(&mut self, ui: Option<&mut egui::Ui>)
    {
        if !self.is_running() { return; }

        // show window nodes
        if ui.is_some() // This check is not entirely needed, as should only execute windows if ui exist, but good for now
        {
            let ui = ui.unwrap();

            for node_key_to_show in self.window_execution.clone()
            {
                self.show_node(&node_key_to_show, ui);
            }
        }

        // update background nodes
        let mut responses = Vec::new();
        for node_key_to_update in self.background_execution.clone()
        {
            let response = self.update_node(&node_key_to_update);
            responses.push(response.clone()); // @TODO, remove this clone and below

            if response.is_none()
            {
                continue; // Change this to return when you move it back!
            }

            self.node_graph.set_output_port_values(&node_key_to_update, &response.unwrap());
            self.node_graph.distribute_outputs_2(&node_key_to_update); 
        }

        // Setup next nodes in execution queue
        if !self.execution_queue.is_empty() // @TODO, change this if statement to return instead
        {
            let node_key_to_setup = self.execution_queue[0];
            let response = self.setup_node(&node_key_to_setup);


            if response.is_some()// change this to return when you move it back
            {
                self.node_graph.set_output_port_values(&node_key_to_setup, &response.as_ref().unwrap());
                self.node_graph.distribute_outputs_2(&node_key_to_setup); 
            }

            if self.debug_mode 
            {
                analysis::runtime_debugging(self);
            }

            self.execution_queue.pop_front(); // @TODO, add a way to remove running nodes
            responses.push(response.clone()); // @TODO, remove this clone and below
        }

        // @TODO, make this work!
        // for output in responses
        // {
        //     if output.is_none()
        //     {
        //         return;
        //     }

        //     let node_key = 0; // fill out!

        //     self.node_graph.set_output_port_values(&node_key, &output.unwrap());
        //     let distribution_result = self.node_graph.distribute_outputs(&node_key); 
        //     self.execution_queue.extend(distribution_result);
        // }
    }

    fn setup_node(&mut self, node_key: &NodeGraphKey) -> Option<Vec<PortValue>>
    {
        let node = self.node_graph.nodes.get_mut(&node_key).unwrap();

        // @TODO, fix this
        match node.kind.function()
        {
            NodeFunction::Window => 
            {
                // @TODO, make it automatically detect this earlier
                if self.window_manager.main_window == None
                {
                    self.window_manager.main_window = Some( *node_key );
                }

                // @TODO, simplify this window manager stuff
                let window_name = self.window_manager.adjust_window_name( node.kind.name() );
                self.window_manager.sub_windows.insert(*node_key, window_name);

                self.window_execution.push(*node_key);
                self.background_execution.push(*node_key);
            },
            _ => {},
        }

        let mut input_port_values = Vec::with_capacity(node.input_port_keys.len());
        for input_port_key in &node.input_port_keys
        {
            let input_port = self.node_graph.input_ports.get(input_port_key).unwrap();
            input_port_values.push(&input_port.value);
        } 

        node.kind.setup(input_port_values, &mut self.log)
    }

    fn update_node(&mut self, node_key: &NodeGraphKey) -> Option<Vec<PortValue>>
    {
        let node = self.node_graph.nodes.get_mut(&node_key).unwrap();
        node.kind.update()
    }

    fn show_node(&mut self, node_key: &NodeGraphKey, ui: &mut egui::Ui)
    {
        let node = self.node_graph.nodes.get_mut(&node_key).unwrap();

        if self.window_manager.main_window.is_none()
        {
            return; // Should only happen on the first update loop
        }

        let main_window_id = self.window_manager.main_window.unwrap();

        if !self.running_in_editor && *node_key == main_window_id
        {
            node.kind.execute(ui);
            return;
        }

        // @TODO, simplify this line
        let window_title = self.window_manager.sub_windows.get(node_key).unwrap();

        let mut window_open = true;
        egui::Window::new(window_title)
        .open(&mut window_open)
        .show(ui.ctx(), |window_ui|
        {
            node.kind.execute(window_ui);
        });

        if window_open == false
        {
            // @TODO, find a better approach here! Also think about the fact that it is modified higher up
            let mut index_to_remove = 0;
            for (index, key) in self.background_execution.iter().enumerate()
            {
                if key == node_key
                {
                    index_to_remove = index;
                    break;
                }
            }

            self.background_execution.remove(index_to_remove);

            for (index, key) in self.window_execution.iter().enumerate()
            {
                if key == node_key
                {
                    index_to_remove = index;
                    break;
                }
            }

            self.window_execution.remove(index_to_remove);
            self.window_manager.sub_windows.remove(node_key);
        }



    }

    // fn setup_node(&mut self, node_key: &NodeGraphKey) -> Option<Vec<NodeGraphKey>>
    // {
    //     if self.debug_mode && self.executing_new_node // This statement is for debug only
    //     {
    //         analysis::runtime_debugging(self);
    //     }

    //     let node_to_execute = self.node_graph.nodes.get_mut(node_key).expect("View node tried to fetch a node that doesn't exist");

    //     let mut input_port_values = Vec::with_capacity(node_to_execute.input_port_keys.len());
    //     for input_port_key in &node_to_execute.input_port_keys
    //     {
    //         let input_port = self.node_graph.input_ports.get(input_port_key).unwrap();
    //         input_port_values.push(&input_port.value);
    //     } 
        
    //     let executed_output_values = node_to_execute.kind.setup(input_port_values)

    //     if executed_output_values.is_none()
    //     {
    //         return None;
    //     }

    //     let output_values = executed_output_values.unwrap();
    //     self.node_graph.set_output_port_values(node_key, &output_values);
        
    //     let distribution_result = self.node_graph.distribute_outputs(node_key); 
    //     Some( distribution_result )
    // }

    // fn execute_node(&mut self, node_key: &NodeGraphKey, ui: Option<&mut egui::Ui>) -> Option<Vec<NodeGraphKey>>
    // {

    //     let node_to_execute = self.node_graph.nodes.get_mut(node_key).expect("View node tried to fetch a node that doesn't exist");

    //     // let input_port_values = self.get_node_input_port_values(node_key).clone(); // @Consider if there is a way to avoid this clone
    //     let mut input_port_values = Vec::with_capacity(node_to_execute.input_port_keys.len());
    //     for input_port_key in &node_to_execute.input_port_keys
    //     {
    //         let input_port = self.node_graph.input_ports.get(input_port_key).unwrap();
    //         input_port_values.push(&input_port.value);
    //     } 

    //     let node_needs_seperate_window = self.window_counter > 1;

    //     let mut executed_output_values = None;

    //     // @TODO, fix these nested if statement
    //     if self.executing_new_node == true
    //     {
    //         self.last_executed_node = *node_key;
    //         self.executing_new_node = false;

    //         match node_to_execute.kind.function()
    //         {
    //             NodeFunction::Window => self.window_counter += 1,
    //             _ => {},
    //         }

    //         executed_output_values = node_to_execute.kind.setup(input_port_values);
    //     }
    //     else 
    //     {
    //         if node_needs_seperate_window
    //         {
    //             let mut window_open = true;

    //             egui::Window::new("Debug Panel")
    //             .open(&mut window_open)
    //             .show(ui.unwrap().ctx(), |window_ui|
    //             {
    //                 executed_output_values = node_to_execute.kind.execute(Some( window_ui ));
    //             });

    //             if !window_open
    //             {
    //                 executed_output_values = Some( Vec::new() );
    //             }
    //         }
    //         else 
    //         {
    //             executed_output_values = node_to_execute.kind.execute(ui);
    //         }
    //     }

        // executed_output_values = 
        // if node_needs_seperate_window
        // {
        //     let ui = ui.expect("Program was instantiated with window nodes, but a ui was not created");

        //     egui::Window::new("Debug Panel")
        //     .show(ui.ctx(), |window_ui|
        //     {
        //         if self.executing_new_node == true
        //         {
        //             self.last_executed_node = *node_key;
        //             self.executing_new_node = false;

        //             match node_to_execute.kind.function()
        //             {
        //                 NodeFunction::Window => self.window_counter += 1,
        //                 _ => {},
        //             }

        //             node_to_execute.kind.setup(input_port_values)
        //         }
        //         else 
        //         {
        //             node_to_execute.kind.execute(window_ui)
        //         };

        //     });
        // }
        // else 
        // {
        //     if self.executing_new_node == true
        //     {
        //         self.last_executed_node = *node_key;
        //         self.executing_new_node = false;

        //         match node_to_execute.kind.function()
        //         {
        //             NodeFunction::Window => self.window_counter += 1,
        //             _ => {},
        //         }

        //         node_to_execute.kind.setup(input_port_values)
        //     }
        //     else 
        //     {
        //         node_to_execute.kind.execute(window_ui)
        //     };

        // };



        // let executed_output_values = if self.last_executed_node != *node_key
        // let executed_output_values = if self.executing_new_node == true
        // {
        //     self.last_executed_node = *node_key;
        //     self.executing_new_node = false;

        //     match node_to_execute.kind.function()
        //     {
        //         NodeFunction::Window => self.window_counter += 1,
        //         _ => {},
        //     }

        //     node_to_execute.kind.setup(input_port_values)
        // }
        // else 
        // {
        //     node_to_execute.kind.execute(ui)
        // };


        // let mut logging = TextBuffer::new();
        // let executed_output_values = node_to_execute.kind.execute(input_port_values, ctx, &mut logging);

//         if executed_output_values.is_none()
//         {
//             return None;
//         }

//         self.executing_new_node = true;

//         let output_values = executed_output_values.unwrap();
//         self.node_graph.set_output_port_values(node_key, &output_values);
        
//         let distribution_result = self.node_graph.distribute_outputs(node_key); 
//         Some( distribution_result )
//     }
// }

// fn handle_node_execution(node: &mut Node, inputs: &Vec<PortValue>, executing_new_node: bool, node_needs_seperate_window: bool, ui: Option< &mut egui::Ui > ) -> Option<Vec<PortValue>>
// {
//     let ui = ui.unwrap();

//     if executing_new_node
//     {
//         // self.last_executed_node = *node_key;
//         // self.executing_new_node = false;

//         match node.kind.function()
//         {
//             NodeFunction::Window => self.window_counter += 1,
//             _ => {},
//         }

//         return node_to_execute.kind.setup(input_port_values);
//     }

//     if !node_needs_seperate_window
//     {
//         return node_to_execute.kind.execute(ui);
//     }

//     let mut window_open = true;

//     egui::Window::new("Debug Panel")
//     .open(&mut window_open)
//     .show(ui.unwrap().ctx(), |window_ui|
//     {
//         return node_to_execute.kind.execute(Some( window_ui ));
//     });

//     if !window_open
//     {
//         return Some( Vec::new() );
//     }

//     None
// }

}

#[derive(Clone)]
pub struct WindowManager // @TODO, pub might not be needed
{
    pub main_window: Option<NodeGraphKey>, // @TODO, make these private
    pub sub_windows: HashMap<NodeGraphKey, String>,
}

impl WindowManager
{
    pub fn new() -> Self
    {
        Self
        {
            main_window: None,
            sub_windows: HashMap::new(),
        }
    }

    pub fn adjust_window_name(&self, node_name: &str) -> String
    {
        let mut number_of_windows_with_same_name = 0;
        for text in self.sub_windows.values()
        {
            if text.contains(node_name)
            {
                number_of_windows_with_same_name += 1;
            }
        }

        if number_of_windows_with_same_name == 0
        {
            return String::from( node_name );
        }

        format!("{} ({})", node_name, number_of_windows_with_same_name)
    } 
}
