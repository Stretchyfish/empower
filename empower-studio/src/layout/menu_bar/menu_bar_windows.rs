use crate::{actions::Action, commands::Command, user_state::UserState};

pub fn show_menu_bar_windows(ui: &mut egui::Ui, command: &mut Command) // @TODO, not a great name
{
    ui.menu_button("Windows", |ui|
    {
        if ui.button("default layout").clicked()
        {
            *command = Command::SetDefaultLayout;
        }
        if ui.button("clear layout").clicked()
        {
            *command = Command::SetClearLayout;
        }
        ui.menu_button("Open Recent", |ui|
        {
            if ui.button("Graph viewport").clicked()
            {
                *command = Command::AddViewport { name: "graph viewport" };
            }
            if ui.button("Terminal Viewport").clicked()
            {
                *command = Command::AddViewport { name: "terminal viewport" };
            }
            if ui.button("Content Browser Viewport").clicked()
            {
                *command = Command::AddViewport { name: "content browser viewport" };
            }
            if ui.button("Empty Viewport").clicked()
            {
                *command = Command::AddViewport { name: "empty viewport" };
            }
        });

        if ui.button("save layout").clicked()
        {
            *command = Command::SaveLayout;
        }

        if ui.button("load layout").clicked()
        {
            *command = Command::LoadLayout;
        }
    });
}
