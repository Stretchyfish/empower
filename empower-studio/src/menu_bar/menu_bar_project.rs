use crate::studio_context::StudioContext;

pub fn show(ui: &mut egui::Ui, _: &mut StudioContext)
{
    ui.menu_button("Project", |ui|
    {
        if ui.button("Save").clicked()
        {
            // studio_context.request_save_project();
        }
        if ui.button("Save As").clicked()
        {
            // studio_context.request_save_project_as();
        }
        if ui.button("Open").clicked()
        {
            // studio_context.request_load_project();
        }
    });
}
