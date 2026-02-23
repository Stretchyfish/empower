use crate::settings::Settings;

pub fn show_menu_bar_settings(ui: &mut egui::Ui, settings: &mut Settings)
{
    ui.menu_button("settings", |ui|
    {
        if ui.button("Developer settings").clicked()
        {
            settings.windows.developer_settings.show = !settings.windows.developer_settings.show;
        }
    });
}
