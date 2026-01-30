use std::{path::PathBuf, str::FromStr};

use eframe::glow::POINT;

use crate::studio_context::StudioContext;


pub fn show(ctx: &egui::Context, studio_context: &mut StudioContext)
{
    if studio_context.layout.project_name_window.is_none()
    {
        return;
    }

    let possible_project_name = studio_context.layout.project_name_window.as_mut().unwrap();

    let mut window_active = true;
    egui::Window::new("New Project Panel")
    .collapsible(false)
    .resizable(false)
    .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
    // .open(&mut window_active)
    .show(ctx, |ui| 
    {

        ui.label("Project Name");
        ui.text_edit_singleline(possible_project_name)
        .request_focus();

        // @TODO, add checks that the name is valid

        ui.horizontal(|ui|
        {
            if ui.button("Cancel").clicked() 
            {
                window_active = false;
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| 
            {
                if ui.button("Create").clicked() 
                {
                    studio_context.project.name = possible_project_name.clone(); // @TODO, make the name assignment more safe
                    studio_context.project.save();
                    window_active = false;
                }
            });
        });

        let escape_pressed = ui.input(|i| i.key_pressed(egui::Key::Escape)); // @TODO, maybe find a better way of handling this?
        if escape_pressed
        {
            window_active = false;
        }

        
        let enter_pressed  = ui.input(|i| i.key_pressed(egui::Key::Enter));
        if enter_pressed
        {
            studio_context.project.name = possible_project_name.clone(); // @TODO, odd code reuse, consider a better way
            studio_context.project.save();
            window_active = false;
        }
    });

    if !window_active
    {
        studio_context.layout.project_name_window = None;
    }
}
