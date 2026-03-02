use crate::{studio_context::StudioContext, user_inputs::UserInputs};

#[derive(Clone)]
pub struct Windows // @TODO, consider a better name, maybe pop ups?
{
    pub project_name_window: ProjectNameWindow,
}

impl Windows
{
    pub fn new() -> Self
    {
        Self
        {
            project_name_window: ProjectNameWindow::new(),
        }
    }
}

#[derive(Clone)]
pub struct ProjectNameWindow
{
    show: bool,
    possible_project_name: String,
}

impl ProjectNameWindow
{
    pub fn new() -> Self
    {
        Self
        {
            show: false,
            possible_project_name: String::new(),
        }
    }

    pub fn activate_show(&mut self, current_project_name: String)
    {
        self.show = true;
        self.possible_project_name = current_project_name;
    }

    pub fn deactivate_show(&mut self)
    {
        self.show = false;
    }

    pub fn show(&mut self, ctx: &egui::Context, studio_context: &mut StudioContext, user_inputs: &UserInputs)
    {
        if !self.show { return; }

        if user_inputs.clicked_esp
        {
            self.show = false;
            return;
        }

        let mut valid_project_name = false;
        let mut project_name_problem_text = String::new();

        match self.is_possible_project_name_valid()
        {
            Ok(_) => valid_project_name = true,
            Err( problem ) => project_name_problem_text = problem,
        }

        if user_inputs.clicked_enter && valid_project_name
        {
            self.enter_behavior(studio_context, &valid_project_name);
        }

        let mut show_copy = self.show.clone();
        egui::Window::new("New Project Panel")
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .open(&mut show_copy)
        .show(ctx, |ui| 
        {

            ui.label("Project Name");
            ui.text_edit_singleline(&mut self.possible_project_name)
            .request_focus();

            ui.horizontal(|ui|
            {
                if ui.button("Cancel").clicked() 
                {
                    self.show = false;
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| 
                {
                    if ui.button("Create").clicked()
                    {
                        self.enter_behavior(studio_context, &valid_project_name);
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
        });

        if show_copy == false
        {
            self.show = false;
        }
    }

    fn is_possible_project_name_valid(&self) -> Result<(), String>
    {
        let name = self.possible_project_name.clone();

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

    fn enter_behavior(&self,studio_context: &mut StudioContext, project_name_valid: &bool)
    {
        if !project_name_valid
        {
            return;
        }

        {
            let project = studio_context.get_project_mut(); // @TODO, this is not a great approach, and should be changed
            project.name = self.possible_project_name.clone();
        }

        studio_context.request_save_project();
    }
}

