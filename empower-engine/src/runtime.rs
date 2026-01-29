use crate::NodeGraph;

pub mod analysis;
mod visualizer;
use visualizer::EmpowerVisualizer;

mod executor;
pub use executor::EmpowerExecutor;

pub struct EmpowerRuntime
{
    node_graph_executor: EmpowerExecutor,
}

impl EmpowerRuntime
{
    pub fn new(node_graph: NodeGraph, debug_mode: bool) -> Self
    {
        let mut node_graph_executor = EmpowerExecutor::new(node_graph, false, debug_mode);
        node_graph_executor.start_node_graph();
 
        Self
        {
            node_graph_executor,
        }
    }

    pub fn execute(&mut self)
    {
        let mut node_graph_uses_graphics = false;
        if self.node_graph_executor.node_graph.contains_node_kind("math graph") ||
            self.node_graph_executor.node_graph.contains_node_kind("show image")
        {
            node_graph_uses_graphics = true;
        }

        if node_graph_uses_graphics == false
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
        
        let _ = eframe::run_native(
            "empower app",
            native_options,
            Box::new(|cc| 
            {
                egui_extras::install_image_loaders(&&cc.egui_ctx);
                Ok(Box::new(EmpowerVisualizer::new(executor)))
            }),
        );
     }

    pub fn execute_with_ui(&mut self, ui: &mut egui::Ui)
    {
        self.node_graph_executor.execute_node_graph(Some( ui ));

    }
}


