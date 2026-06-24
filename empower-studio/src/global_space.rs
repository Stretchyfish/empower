use crate::studio_context::StudioContext;

mod developer_panel;
pub use developer_panel::DeveloperPanel;

pub fn show(ui: &mut egui::Ui, studio_context: &mut StudioContext)
{
    let (windows, settings, assets) = studio_context.get_windows_and_settings_mut_and_assets();

    // Each window show function does itself keep track of it should have an open window or not!
    windows.developer_panel.show(ui, settings, assets);

}
