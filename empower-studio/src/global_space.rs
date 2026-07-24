use crate::{studio_context::StudioContext, user_inputs::UserInputs};

mod developer_panel;
pub use developer_panel::DeveloperPanel;

mod project_naming_window;
pub use project_naming_window::ProjectNameWindow;

mod export_panel;
pub use export_panel::ExportPanel;

pub fn show(ui: &mut egui::Ui, studio_context: &mut StudioContext, user_inputs: &UserInputs)
{
    let mut windows = studio_context.get_windows().clone(); // @TODO, this can be potentially expensive, think of a better way
    
    // Each window show function does itself keep track of it should have an open window or not!
    windows.developer_panel.show(ui, studio_context);
    windows.project_name_panel.show(ui, studio_context, user_inputs);
    windows.export_panel.show(ui, studio_context);

    studio_context.set_windows(windows);

}
