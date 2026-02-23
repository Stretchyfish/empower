use crate::{actions::Action, commands::Command, user_state::UserState};

pub fn show_menu_bar_project(ui: &mut egui::Ui, action_queue: &mut Vec<Action>, command: &mut Command)
{
    ui.menu_button("Project", |ui|
    {
        if ui.button("Save").clicked()
        {
            *command = Command::SaveProject;
        }
        if ui.button("Save As").clicked()
        {
            *command = Command::SaveProjectAs;
            // action_queue.push( Action::SaveProjectAs );
        }
        if ui.button("Open").clicked()
        {
            *command = Command::OpenProject;
            let project_path = rfd::FileDialog::new()
                                                .set_title("Choose project location")
                                                .set_can_create_directories(true)
                                                .pick_folder();
            action_queue.push( Action::LoadProject { project_path: project_path.unwrap() } ); // @TODO, this is dangerous
        }
        ui.menu_button("Open Recent", |ui|
        {
            // for previous_project in &studio_context.layout.previous_projects
            // {
            //     if ui.button(previous_project.project_name.to_string()).clicked()
            //     {

            //         action_queue.push( Action::LoadProject { project_path: previous_project.project_location.clone() } ); // @TODO, this is dangerous
                    
            //     }
            // }
        });
        if ui.button("Project Settings").clicked()
        {
            // studio_context.layout.project_setting_window = !studio_context.layout.project_setting_window;
        }
        if ui.button("Export").clicked()
        {
            
        }
    });
}
