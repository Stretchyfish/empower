use super::EmpowerExecutor;

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
        if !self.node_graph_executor.is_running()
        {
            // Implement termination behavior here
        }

        egui::CentralPanel::default()
            .frame(egui::Frame::central_panel(&ctx.style()).inner_margin(0.))
            .show(ctx, |ui| 
        {
            self.node_graph_executor.execute_node_graph( Some( ui ));
        });
    }
}
