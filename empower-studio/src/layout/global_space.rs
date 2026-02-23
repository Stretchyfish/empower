use crate::{actions::Action, commands::Command, settings::Settings, user_inputs::UserInputs, user_state::UserState};

pub fn show_global_space(ctx: &egui::Context, settings: &mut Settings, user_state: &mut UserState, user_inputs: &UserInputs, command: &mut Command)
{
    process_global_user_inputs(ctx, user_inputs, command);

    match user_state.clone()
    {
        UserState::Idle => {},
        UserState::DraggingAsset => {},
        UserState::NamingProject { new_project_name } => { show_naming_project_window(ctx, user_state, user_inputs, &mut new_project_name); },
    };

    if settings.windows.developer_settings.show
    {
        settings.windows.developer_settings.show(ctx);
    }
}

fn process_global_user_inputs(ctx: &egui::Context, user_inputs: &UserInputs, command: &mut Command)
{
    let save_requested = user_inputs.holding_ctrl && user_inputs.clicked_s;

    if save_requested
    {
        *command = Command::SaveProject;
    }
}

fn show_naming_project_window(ctx: &egui::Context, user_state: &mut UserState,user_inputs: &UserInputs, possible_project_name: &mut String)
{
    let mut keep_window_open = true;

    if user_inputs.clicked_esp
    {
        keep_window_open = false;
    }

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


    if !keep_window_open
    {
        *user_state = UserState::Idle;
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

