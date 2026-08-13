use std::{path::PathBuf, sync::Arc, thread::{self, JoinHandle}};

use egui::mutex::Mutex;
use rfd::FileDialog;

use crate::{studio_context::{Log, StudioContext}, user_inputs::UserInputs};

#[derive(Clone)]
pub struct ProjectNameWindow
{
    show: bool,
    possible_project_name: String,
    task: Arc<Mutex<Option<JoinHandle<Option<PathBuf>>>>> // @TODO, this gets so complicated because of the clone, think in the future of a way to improve
}

impl ProjectNameWindow
{
    pub fn new() -> Self
    {
        Self
        {
            show: false,
            possible_project_name: String::new(),
            task: Arc::new(Mutex::new(None)),
        }
    }

    pub fn activate_show(&mut self, current_project_name: String)
    {
        self.show = true;
        self.possible_project_name = current_project_name;
    }

    pub fn show(&mut self, ui: &mut egui::Ui, studio_context: &mut StudioContext, user_inputs: &UserInputs)
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
            Ok(_) => { valid_project_name = true; },
            Err( problem ) => { project_name_problem_text = problem; },
        }

        if user_inputs.clicked_enter && valid_project_name
        {
            self.enter_behavior(studio_context, &valid_project_name);
            return;
        }

        self.show_project_location_dialog(studio_context);

        let mut show_copy = self.show.clone();
        egui::Window::new("New Project Panel")
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .open(&mut show_copy)
        .show(ui, |ui| 
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

        // let mut problems = String::new();

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

    fn enter_behavior(&mut self, studio_context: &mut StudioContext, project_name_valid: &bool)
    {
        if !project_name_valid
        {
            return;
        }

        {
            studio_context.get_project_mut().name = self.possible_project_name.clone();
        }

        self.start_project_location_dialog();

        // studio_context.request_save_project();
        // self.show = false;
    }

    fn start_project_location_dialog(&mut self)
    {
        let mut task = self.task.lock(); // @TODO, this seems potentially very unsafe, investigate better approaches
        
        if task.is_some() // Don't want two dialogs spawned at once
        {
            return;
        }

        *task = Some( thread::spawn(move || {
            FileDialog::new()
            .set_title("import asset")
            .pick_folder()
        }));
    }

    fn show_project_location_dialog(&mut self, studio_context: &mut StudioContext)
    {
        let mut task = self.task.lock();

        if task.is_none()
        {
            return;
        }

        let task_finished = task.as_ref().unwrap().is_finished();

        if !task_finished
        {
            return;
        }

        match task.take().unwrap().join()
        {
            Ok( result ) =>
            {
                match result
                {
                    Some( path ) =>
                    {
                        studio_context.request_save_project( Some( path ) );
                        self.show = false;
                    },
                    None =>
                    {
                        panic!("Unable to handle process correctly");
                    },
                }
            },
            Err(_) =>
            {
                studio_context.add_log( Log::info( "cannot read path when saving project" ) );
            },
        }
    }
}


