use crate::{studio_context::StudioContext, user_inputs::UserInputs};

mod developer_panel;
pub use developer_panel::DeveloperPanel;

mod project_naming_window;
pub use project_naming_window::ProjectNameWindow;

mod export_panel;
pub use export_panel::ExportPanel;

mod asset_import_dialog;
pub use asset_import_dialog::AssetImportDialog;

mod dragged_asset;

mod logging_space;


pub fn show(ui: &mut egui::Ui, studio_context: &mut StudioContext, user_inputs: &UserInputs)
{
    detect_global_hotkeys(studio_context, user_inputs);
    
    let mut windows = studio_context.get_windows().clone(); // @TODO, this can be potentially expensive, think of a better way
    
    // Each window show function does itself keep track of it should have an open window or not!
    windows.developer_panel.show(ui, studio_context);
    windows.project_name_panel.show(ui, studio_context, user_inputs);
    windows.export_panel.show(ui, studio_context);
    windows.asset_import_dialog.show(studio_context);

    studio_context.set_windows(windows);

    dragged_asset::show(ui, studio_context);
    logging_space::show(studio_context.get_logs(), ui);
}

fn detect_global_hotkeys(studio_context: &mut StudioContext, user_inputs: &UserInputs)
{
    if user_inputs.holding_ctrl && user_inputs.clicked_s
    {
        studio_context.request_save_project(None);
    }
    
    if user_inputs.holding_alt && user_inputs.clicked_d
    {
        studio_context.get_settings_mut().developer_mode = !studio_context.get_settings_mut().developer_mode;
    }
}
