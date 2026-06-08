use crate::studio_context::StudioContext;

pub fn show(ui: &mut egui::Ui, studio_context: &mut StudioContext)
{
    let windows = studio_context.get_windows_mut();
    
    ui.menu_button("Settings", |ui|
    {
        if ui.button("developer panel").clicked()
        {
            windows.developer_panel.toggle_show();
        }
    });
}
