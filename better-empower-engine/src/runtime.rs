use crate::{NodeGraph, NodeGraphKey, node_graph};

use node_graph::node::NodeFunction;

use std::collections::VecDeque;

mod analysis;

pub struct EmpowerRuntime
{
    node_graph_executor: EmpowerExecutor,
    // window_handler: EmpowerWindowHandler,
    // last_executed_node: NodeGraphKey, // @TODO, consider doing this as a clone, to avoid changed persisting between execution in studio
    // execution_queue: VecDeque<NodeGraphKey>,
    // log: TextBuffer,
 
}

impl EmpowerRuntime
{
    pub fn new(node_graph: NodeGraph) -> Self
    {
        let mut node_graph_executor = EmpowerExecutor::new(node_graph);
        node_graph_executor.start_node_graph();
 
        Self
        {
            node_graph_executor,
        }
    }

    pub fn execute(&mut self)
    {
        let mut window_counter = 0;
        for node in self.node_graph_executor.node_graph.nodes.values() // @TODO, find a better approach
        {
            match node.kind.function()
            {
                NodeFunction::Window => window_counter += 1,
                _ => {},
            }
        }

        if window_counter == 0
        {
            while self.node_graph_executor.is_running()  
            {
                self.node_graph_executor.execute_node_graph(None);
            }

            return;
        }

        let viewport_builder = egui::ViewportBuilder::default()
        .with_always_on_top()
        .with_active(true)
        .with_clamp_size_to_monitor_size(true)
        .with_inner_size(egui::Vec2 { x: 1920.0, y: 1080.0 })
        .with_maximized(true); // @TODO, improve the maximized approach
        
        let native_options = eframe::NativeOptions { 
                                                        vsync: false, 
                                                        viewport: viewport_builder,
                                                        ..Default::default()};

        let executor = self.node_graph_executor.clone(); // @TODO, this can be potentially be a very expensive call, find a better way
        
        eframe::run_native(
            "empower app",
            native_options,
            Box::new(|_| Ok(Box::new(EmpowerVisualizer::new(executor)))),
        );
     }

    pub fn execute_with_ui(&mut self, ui: &mut egui::Ui)
    {
        self.node_graph_executor.execute_node_graph(Some( ui ));

    }
}

pub struct EmpowerVisualizer
{
    node_graph_executor: EmpowerExecutor,
}

impl EmpowerVisualizer
{
    pub fn new(node_graph_executor: EmpowerExecutor) -> Self
    {
        Self
        {
            node_graph_executor,
        }
    }
}

impl eframe::App for EmpowerVisualizer
{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame)
    {
        egui::CentralPanel::default()
            .frame(egui::Frame::central_panel(&ctx.style()).inner_margin(0.))
            .show(ctx, |ui| 
        {
            self.node_graph_executor.execute_node_graph( Some( ui ));
        });
    }
}

#[derive(Clone)]
pub struct EmpowerExecutor
{
    node_graph: NodeGraph,
    last_executed_node: NodeGraphKey, // @TODO, consider doing this as a clone, to avoid changed persisting between execution in studio
    execution_queue: VecDeque<NodeGraphKey>,
    // log: TextBuffer,
}

impl EmpowerExecutor
{
    pub fn new(node_graph: NodeGraph) -> Self
    {
        Self
        {
            node_graph,
            last_executed_node: 0, // @TODO, find a better approach, its currently set to 0, because 0 is unsued
            execution_queue: VecDeque::new(),
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
        self.execution_queue.clear();

        let rouge_nodes = analysis::detect_rouge_nodes(&self.node_graph);

        self.execution_queue.extend(rouge_nodes);

        if *node_key == 1 // Start node key (This check is only for debugging of node graph without a start node, should get removed later)
        {
            let start_node = self.node_graph.nodes.get(node_key).unwrap();

            if start_node.kind.name() == "start"
            {
                self.execution_queue.push_back(*node_key);
            }
        }
    }

    pub fn stop_node_graph(&mut self)
    {
        self.execution_queue.clear();
    }

    pub fn is_running(&self) -> bool
    {
       !self.execution_queue.is_empty() 
    }

    fn execute_node_graph(&mut self, ui: Option<&mut egui::Ui>)
    {
        if self.execution_queue.is_empty() { return; }

        let node_to_execute = self.execution_queue[0]; // @TODO, consider if it should be rewritten with .front instead

        let execution_response = self.execute_node(&node_to_execute, ui);

        if execution_response.is_none()
        {
            return;
        }

        self.execution_queue.extend(execution_response.unwrap());
        self.execution_queue.pop_front();
    }

    fn execute_node(&mut self, node_key: &NodeGraphKey, ui: Option<&mut egui::Ui>) -> Option<Vec<NodeGraphKey>>
    {
        let node_to_execute = self.node_graph.nodes.get_mut(node_key).expect("View node tried to fetch a node that doesn't exist");

        // let input_port_values = self.get_node_input_port_values(node_key).clone(); // @Consider if there is a way to avoid this clone
        let mut input_port_values = Vec::with_capacity(node_to_execute.input_port_keys.len());
        for input_port_key in &node_to_execute.input_port_keys
        {
            let input_port = self.node_graph.input_ports.get(input_port_key).unwrap();
            input_port_values.push(&input_port.value);
        } 

        let executed_output_values = if self.last_executed_node != *node_key
        {
            self.last_executed_node = *node_key;
            node_to_execute.kind.setup(input_port_values)
        }
        else 
        {
            node_to_execute.kind.execute(ui)
        };

        // let mut logging = TextBuffer::new();
        // let executed_output_values = node_to_execute.kind.execute(input_port_values, ctx, &mut logging);

        if executed_output_values.is_none()
        {
            return None;
        }

        let output_values = executed_output_values.unwrap();
        self.node_graph.set_output_port_values(node_key, &output_values);
        
        let distribution_result = self.node_graph.distribute_outputs(node_key); 
        Some( distribution_result )
    }

}

