use egui;
use egui_extras;

mod project;
use project::{Project, ProjectState};

mod settings;
use settings::Settings;

mod user_state;
use user_state::UserState;

mod actions;
use actions::Action;

mod layout;
use layout::Layout;

mod user_inputs;

mod commands;
use commands::Command;

mod cache;
use cache::Cache;

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
    // studio_context: StudioContext,

    project: Project,
    settings: Settings,
    layout: Layout,

    cache: Cache,

    user_state: UserState,
}

impl EmpowerStudioApplication
{
    pub fn new() -> Self
    {
        Self 
        {
            // studio_context: StudioContext::new(),

            project: Project::new(),
            settings: Settings::new(),
            layout: Layout::load(), // Tries first to load a saved layout if one exists

            cache: Cache::new(), // @TODO, this should have a load

            user_state: UserState::Idle, // Initial state
        }
    }
}

impl eframe::App for EmpowerStudioApplication
{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame)
    {
        // let mut action_queue = Vec::new();

        // workspace::show(ctx, &mut self.studio_context, &mut action_queue);

        // self.studio_context.process_actions(action_queue, ctx);

        // return;

        // processes::background_processes();

        // user_state::process_user_state(&mut self.user_state);

        let mut command = Command::None; // @TODO, consider making this a struct instead with method to change

        let user_inputs = user_inputs::get_user_inputs(ctx);
        let mut new_action_queue: Vec<Action> = Vec::new(); // @TODO, make a better object for action handling

        self.layout.show_menu_bar(ctx, &mut self.settings, &mut new_action_queue, &mut command);
        self.layout.show_global_space(ctx, &mut self.settings, &mut self.user_state, &user_inputs, &mut new_action_queue, &mut command);
        self.layout.show_docking_space(ctx, &mut self.project, &self.settings, &self.user_state, &user_inputs, &mut new_action_queue, &mut command);

        commands::process_command(command, &mut self.layout, &mut self.project, &mut self.user_state);

        // self.process_actions(ctx, new_action_queue);

        
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>)
    {
        self.layout.save();
        // self.studio_context.layout.save();
    }
}

