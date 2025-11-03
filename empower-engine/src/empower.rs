use crate::{node_graph, NodeGraph, NodeGraphKey};

pub struct Empower
{
}

impl Empower
{
    pub fn new() -> Self
    {
        Self {  }
    }

    pub fn execute(&mut self, node_graph: NodeGraph)
    {
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
        
        eframe::run_native(
            "empower",
            native_options,
            Box::new(|_| Ok(Box::new(EmpowerRuntime::new(node_graph)))),
        );
    }

    pub fn execute_with_ui(&mut self, ui: &mut egui::Ui, node_graph: &mut NodeGraph)
    {
        update_node_graph(node_graph, ui.ctx());
    }

    pub fn execute_node_with_ui(&mut self, ui: &mut egui::Ui, node_graph: &mut NodeGraph, node_key: &NodeGraphKey)
    {
        
    }
}

pub struct EmpowerRuntime
{
    node_graph: NodeGraph,
}

impl EmpowerRuntime
{
    pub fn new(node_graph: NodeGraph) -> Self
    {
        Self 
        {  
            node_graph,
        }
    }
}

impl eframe::App for EmpowerRuntime
{
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) 
    {
        update_node_graph(&mut self.node_graph, ctx);
    }
}

fn update_node_graph(node_graph: &mut NodeGraph, ctx: &egui::Context)
{

}

fn update_node_graph_from_entry(node_graph: &mut NodeGraph, ctx: &egui::Context, node_key: &NodeGraphKey)
{

}

fn update_node_in_node_graph(node_graph: &mut NodeGraph, ctx: &egui::Context, node_key: &NodeGraphKey)
{

}
