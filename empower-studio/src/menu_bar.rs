use crate::studio_context::StudioContext;

mod menu_bar_project;
mod menu_bar_settings;
mod menu_bar_windows;
mod start_stop_button;

pub fn show(ctx: &egui::Context, studio_context: &mut StudioContext)
{
    egui::TopBottomPanel::top("menu bar").show(ctx, |ui| 
    {
        egui::MenuBar::new()
        .ui(ui, |ui|
        {
            menu_bar_project::show(ui, studio_context);
            menu_bar_settings::show(ui, studio_context);
            menu_bar_windows::show(ui, studio_context);
        });

        ui.horizontal(|ui|
        {
            start_stop_button::show(ui, studio_context);
            // start_stop_button::show_start_stop_button(ui, action_queue);

            // // ui.checkbox(&mut studio_context.project.graph_editor.debug_info.show_keys, "show keys");
            // ui.checkbox(&mut false, "show keys");

            // ui.add_space(20.0);

            // execution_history_visual::show_execution_history_button(ui, action_queue);
        });

        ui.add_space(2.0);
    });
}
