use egui;
use egui_extras;

mod user_inputs;

mod studio_context;
use studio_context::StudioContext;

mod menu_bar;
mod global_space;
mod docking_space;

fn main() -> Result<(), eframe::Error>
{
    let app_image = image::load_from_memory( include_bytes!("../../media/empower_icon.png"))
    .expect("Failed to load icon")
    .into_rgba8();
    let (app_image_width, app_image_height) = app_image.dimensions();

    let app_icon = egui::IconData
    {
        rgba: app_image.into_raw(),
        width: app_image_width,
        height: app_image_height,
    };

    let viewport_builder = egui::ViewportBuilder::default()
    .with_always_on_top()
    .with_active(true)
    .with_clamp_size_to_monitor_size(true)
    .with_inner_size(egui::Vec2 { x: 1920.0, y: 1080.0 })
    .with_icon(app_icon)
    .with_maximized(true);
    
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
        let user_inputs = user_inputs::get_user_inputs(ctx);

        menu_bar::show(ctx, &mut self.studio_context);
        docking_space::show(ctx, &mut self.studio_context, &user_inputs);
        global_space::show(ctx, &mut self.studio_context, &user_inputs);

        self.studio_context.process_requests();
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>)
    {
        self.studio_context.request_save();
        self.studio_context.process_requests(); // Ensures everything is shut down in the correct order
    }
}

