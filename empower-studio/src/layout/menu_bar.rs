use crate::actions::Action;

mod menu_bar_project;
mod menu_bar_settings;
mod menu_bar_windows;
mod start_stop_button;
mod execution_history_visual;

pub fn show_menu_bar(ui: &mut egui::Ui, action_queue: &mut Vec<Action>)
{
    egui::MenuBar::new()
    .ui(ui, |ui|
    {
        menu_bar_project::show_menu_bar_project(ui, action_queue);
        menu_bar_settings::show_menu_bar_settings(ui, action_queue);
        menu_bar_windows::show_menu_bar_windows(ui, action_queue); // @TODO, maybe change this name to layout?
    });

    ui.horizontal(|ui|
    {
        start_stop_button::show_start_stop_button(ui, action_queue);

        // ui.checkbox(&mut studio_context.project.graph_editor.debug_info.show_keys, "show keys");
        ui.checkbox(&mut false, "show keys");

        ui.add_space(20.0);

        execution_history_visual::show_execution_history_button(ui, action_queue);
    });

    ui.add_space(2.0);
}
