use crate::NodeGraph;

pub mod analysis;
mod visualizer;
use visualizer::EmpowerVisualizer;

// mod executor;
// pub use executor::EmpowerExecutor;
//
mod executor2;
pub use executor2::EmpowerExecutor;

pub struct EmpowerRuntime // @TODO, consider making this a pure function
{

}

impl EmpowerRuntime
{
    pub fn new() -> Self
    {
        Self
        {

        }
    }

    pub fn execute(&mut self, node_graph: NodeGraph)
    {
        let mut node_graph_executor = EmpowerExecutor::new(false);
        node_graph_executor.start();
 
        let node_graph_uses_graphics = node_graph.uses_graphics();

        match node_graph_uses_graphics
        {
            false => self.run_empower_headless(node_graph_executor, node_graph),
            true => self.run_empower_graphical(node_graph_executor, node_graph),
        }
     }

     fn run_empower_headless(&self, mut node_graph_executor: EmpowerExecutor, mut node_graph: NodeGraph)
     {
        while node_graph_executor.is_running()  
        {
            node_graph_executor.execute(&mut node_graph, None );
        }
     }

     fn run_empower_graphical(&self, node_graph_executor: EmpowerExecutor, node_graph: NodeGraph)
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

        let _ = eframe::run_native(
            "empower app",
            native_options,
            Box::new(|cc| 
            {
                egui_extras::install_image_loaders(&&cc.egui_ctx);
                Ok(Box::new(EmpowerVisualizer::new(node_graph, node_graph_executor)))
            }),
        );
     }
}


