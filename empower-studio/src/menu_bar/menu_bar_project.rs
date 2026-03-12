use crate::studio_context::StudioContext;

pub fn show(ui: &mut egui::Ui, studio_context: &mut StudioContext)
{
    ui.menu_button("Project", |ui|
    {
        if ui.button("Save").clicked()
        {
            studio_context.request_save_project();
        }
        if ui.button("Save As").clicked()
        {
            studio_context.request_save_project_as();
        }
        if ui.button("Open").clicked()
        {
            studio_context.request_load_project();
        }
        ui.menu_button("Open Recent", |ui|
        {
            for project in studio_context.get_cache().previous_projects.clone()
            {
                if ui.button(project.file_name().unwrap().to_string_lossy().to_string()).clicked()
                {
                    studio_context.request_load_specific_project(project);
                }
            }
        });
        if ui.button("Project Settings").clicked()
        {
            studio_context.get_settings_mut().project_settings.toggle_show();
        }
        if ui.button("Export").clicked()
        {
        
        }
    });
}
