use crate::studio_context::StudioContext;

mod menu_bar_project;
mod menu_bar_settings;
mod menu_bar_windows;
mod menu_bar_export;

mod start_stop_button;
mod compile_button;
mod bar_debug_settings;
mod start_graph_selector;

pub fn show(ui: &mut egui::Ui, studio_context: &mut StudioContext)
{
    egui::Panel::top("menu bar").show_inside(ui, |ui|
    {
        egui::MenuBar::new().ui(ui, |ui|
        {
            menu_bar_project::show(ui, studio_context);
            menu_bar_settings::show(ui, studio_context);
            menu_bar_windows::show(ui, studio_context);
            menu_bar_export::show(ui, studio_context);
        });

        ui.horizontal(|ui|
        {
            start_stop_button::show(ui, studio_context);
            compile_button::show(ui, studio_context);
            start_graph_selector::show(ui, studio_context);
            bar_debug_settings::show(ui, studio_context); // @TODO, change this name
        });

        ui.add_space(2.0);
    });
}
