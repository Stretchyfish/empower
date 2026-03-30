use crate::studio_context::StudioContext;

pub fn show(ui: &mut egui::Ui, studio_context: &mut StudioContext)
{
    let settings = studio_context.get_settings_mut();
   
    ui.menu_button("Settings", |ui|
    {
        if ui.button("Developer settings").on_hover_text("alt + d").clicked()
        {
            settings.developer_settings.toggle_show();
        }
    });
}
