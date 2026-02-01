use std::path::PathBuf;

use egui;

use crate::studio_context::StudioContext;

pub fn show(ctx: &egui::Context, studio_context: &mut StudioContext)
{
    let mut window_active = studio_context.layout.project_setting_window;
    egui::Window::new("Project Setting Panel")
    .collapsible(false)
    .resizable(false)
    .open(&mut window_active)
    .show(ctx, |ui| 
    {

        ui.horizontal(|ui|
        {
            ui.label("Project name");
            ui.text_edit_singleline(&mut studio_context.project.name);
        });

        ui.horizontal(|ui|
        {
            ui.label("Location");

            match &mut studio_context.project.state
            {
                crate::project::ProjectState::Temporary => 
                {
                    if ui.button("Unsaved*").clicked()
                    {
                        let save_folder = rfd::FileDialog::new()
                                        .set_title("Choose project location")
                                        .set_can_create_directories(true)
                                        .pick_folder();
                        studio_context.project.state = crate::project::ProjectState::Saved(save_folder.unwrap());
                    }
                },
                crate::project::ProjectState::Saved( path ) => 
                {
                    let mut path_string = path.clone().to_str().unwrap().to_string();
                    let text_edit_widget = egui::TextEdit::singleline( &mut path_string ) // @TODO, look into this again
                    .interactive(false);

                    ui.add(text_edit_widget);

                    if ui.button("Folder Select").clicked()
                    {
                        let save_folder = rfd::FileDialog::new()
                                        .set_title("Choose project location")
                                        .set_can_create_directories(true)
                                        .pick_folder();
                        studio_context.project.state = crate::project::ProjectState::Saved(save_folder.unwrap());
                    }
                },
            }

        });

        if ui.button("Save").clicked()
        {
            
        }
    });

    studio_context.layout.project_setting_window = window_active;
}

