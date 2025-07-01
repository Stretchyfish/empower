mod graph_editor;

mod studio_context;
use studio_context::StudioContext;

mod workspace;

fn main() -> Result<(), eframe::Error>
{
    let native_options = eframe::NativeOptions { vsync: false, ..Default::default()};

    eframe::run_native(
        "empower studio",
        native_options,
        Box::new(|_| Ok(Box::new(EmpowerStudioApplication::new()))),
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
