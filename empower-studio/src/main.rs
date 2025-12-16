use egui;
use egui_extras;

mod studio_context;
use studio_context::StudioContext;

mod graph_editor;
use graph_editor::GraphEditor;

mod workspace;

fn main() -> Result<(), eframe::Error>
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
        "empower studio",
        native_options,
        Box::new(|cc| 
            {
                egui_extras::install_image_loaders(&&cc.egui_ctx);
                Ok(Box::new(EmpowerStudioApplication::new()))
            }
        ),
    )
}

pub struct EmpowerStudioApplication
{
    studio_context: StudioContext,
}

impl EmpowerStudioApplication
{
    pub fn new() -> Self
    {
        Self 
        {
            studio_context: StudioContext::new(),
        }
    }
}

impl eframe::App for EmpowerStudioApplication
{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame)
    {
        workspace::show(ctx, &mut self.studio_context);        
    }
}
