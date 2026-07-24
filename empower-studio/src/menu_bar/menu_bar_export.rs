use crate::studio_context::StudioContext;


pub fn show(ui: &mut egui::Ui, studio_context: &mut StudioContext)
{
    if ui.button("Export").clicked()
    {
        let windows = studio_context.get_windows_mut();
        windows.export_panel.toggle_show();
    }
}
