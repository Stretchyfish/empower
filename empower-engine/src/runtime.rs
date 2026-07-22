use crate::{compiler::Program, executor::{Executor, ExecutorSettings}};

pub fn run_cli(program: Program)
{
    let mut executor = Executor::new(program, ExecutorSettings::new());

    if !executor.is_running()
    {
        return;
    }

    loop
    {
        executor.run(None);
    }
}

pub fn run_gui(program: Program)
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
                                                ..Default::default() };

    let _ = eframe::run_native(
        "empower app",
        native_options,
        Box::new(|cc| 
        {
            egui_extras::install_image_loaders(&&cc.egui_ctx);
            Ok(Box::new(EmpowerVisualizer::new(program)))
        }),
    );
}

pub struct EmpowerVisualizer
{
    executor: Executor,
}

impl EmpowerVisualizer
{
    pub fn new(program: Program) -> Self
    {
        let settings = ExecutorSettings::new();
        
        Self
        {
            executor: Executor::new(program, settings),
        }
    }
}

impl eframe::App for EmpowerVisualizer
{
    fn ui(&mut self, ui: &mut egui::Ui, _: &mut eframe::Frame)
    {
        if !self.executor.is_running()
        {
            // Implement termination behavior here
            return;
        }

        self.executor.run(Some( ui ));
    }
}
