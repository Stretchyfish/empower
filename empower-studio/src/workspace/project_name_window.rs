use crate::{actions::Action, studio_context::StudioContext};

pub fn show(ctx: &egui::Context, studio_context: &mut StudioContext, action_queue: &mut Vec<Action>)
{
    if studio_context.layout.project_name_window.is_none()
    {
        return;
    }

    let possible_project_name = studio_context.layout.project_name_window.as_mut().unwrap();

    let mut valid_project_name = false;
    let mut project_name_problem_text = String::new();

    match is_project_name_valid(&possible_project_name)
    {
        Ok(_) => valid_project_name = true,
        Err( problem ) => project_name_problem_text = problem,
    }
        
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
                if ui.button("Create").clicked() && valid_project_name
                {
                    // @TODO, remove this (Its a copy of what is below aswel)
                    studio_context.project.name = possible_project_name.clone(); // @TODO, make the name assignment more safe
                    studio_context.project.save();

                    // @TODO, this is double work due to the inconsistend saving behavior, has to get fixed
                    action_queue.push( Action::SaveProject );
                    window_active = false;
                }
            });
        });

        ui.add_space(5.0);

        if valid_project_name
        {
            ui.label(egui::RichText::new("Valid project name").color(egui::Color32::DARK_GREEN));
        }
        else
        {
            ui.label(egui::RichText::new(format!("Invalid project name ({})", project_name_problem_text)).color(egui::Color32::DARK_RED));
        }

        let escape_pressed = ui.input(|i| i.key_pressed(egui::Key::Escape)); // @TODO, maybe find a better way of handling this?
        if escape_pressed
        {
            window_active = false;
        }
        
        let enter_pressed  = ui.input(|i| i.key_pressed(egui::Key::Enter));
        if enter_pressed && valid_project_name
        {
            // @TODO, this should not be allowed!
            studio_context.project.name = possible_project_name.clone(); // @TODO, odd code reuse, consider a better way
            studio_context.project.save();

            // @TODO, this is double work due to the inconsistend saving behavior, has to get fixed
            action_queue.push( Action::SaveProject );

            window_active = false;
        }
    });

    if !window_active
    {
        studio_context.layout.project_name_window = None;
    }
}

fn is_project_name_valid(name: &String) -> Result<(), String>
{
    let mut problems = String::new();

    if name.chars().any(|c| c.is_ascii_uppercase() )
    {
        return Result::Err(String::from("Contains upper case letters"));
    }

    if name.chars().any(|c| c.is_ascii_whitespace() )
    {
        return Result::Err(String::from("Contains white spaces"));
    }

    if name.chars().any(|c| !c.is_ascii_alphanumeric() )
    {
        return Result::Err(String::from("Contains special characters"));
    }

    Result::Ok(())
}
