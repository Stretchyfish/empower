use std::{collections::BTreeMap, path::PathBuf};

use empower_engine::distribution;

use crate::studio_context::StudioContext;

#[derive(Clone)]
pub struct ExportPanel
{
    show: bool,
    export_config: distribution::ExportConfig,

    export_platforms_available: BTreeMap<String, PathBuf>,

    checked_atleast_once_for_runtimes: bool,
}

impl ExportPanel
{
    pub fn new() -> Self
    {
        Self
        {
            show: false,

            export_config: distribution::ExportConfig::new(),
            export_platforms_available: BTreeMap::new(),

            checked_atleast_once_for_runtimes: false,
        }
    }

    pub fn toggle_show(&mut self)
    {
        self.show = !self.show;
    }

    pub fn show(&mut self, ui: &mut egui::Ui, studio_context: &mut StudioContext)
    {
        if !self.show
        {
            return;
        }

        if !self.checked_atleast_once_for_runtimes
        {
            self.check_for_available_export_platforms();

            self.checked_atleast_once_for_runtimes = true;
        }

        egui::Window::new("Export Panel")
        .collapsible(false)
        .resizable(true)
        .auto_sized()
        .open(&mut self.show)
        .show(ui, |ui| 
        {
            ui.label(format!("Default name: {}", studio_context.get_project().name));

            ui.horizontal(|ui|
            {
                ui.checkbox(&mut self.export_config.use_custom_export_name, "custom name");

                if self.export_config.use_custom_export_name
                {
                    let text_edit = egui::TextEdit::singleline(&mut self.export_config.custom_export_name);
                    ui.add( text_edit);
                }
            });

            ui.horizontal(|ui|
            {
                ui.label("Platform: ");

                let selected_platform_name = if self.export_config.platform.is_none()
                {
                    String::from("undefined")
                }
                else
                {
                    self.export_config.platform.clone()
                                                    .unwrap()
                                                    .file_name()
                                                    .unwrap()
                                                    .to_string_lossy()
                                                    .to_string()
                };
            
                ui.menu_button(selected_platform_name, |ui|
                {
                    for (platform_name, platform_path) in &self.export_platforms_available
                    {
                        if ui.button(platform_name).clicked()
                        {
                            self.export_config.platform = Some( platform_path.clone() );
                        }
                    }
                });
            });

            ui.horizontal(|ui|
            {
                ui.label("Application type: ");

                let current_application_type = self.export_config.application_type.to_string();
               
                ui.menu_button(current_application_type, |ui|
                {
                    if ui.button("Graphical").clicked()
                    {
                        self.export_config.application_type = distribution::ApplicationType::Graphical;
                    }

                    if ui.button("CLI").clicked()
                    {
                        self.export_config.application_type = distribution::ApplicationType::CLI;
                    }
                });
            });

            ui.horizontal(|ui|
            {
                ui.label("Export location: ");
                if ui.button("choose location").clicked()
                {
                    
                    let folder_path = rfd::FileDialog::new()
                                                        .set_title("Choose export location")
                                                        .set_can_create_directories(true)
                                                        .pick_folder();

                    if folder_path.is_none()
                    {
                        println!("Failed to get folder path!"); // @TODO, in the future, handle this error properly!
                        return;
                    }

                    self.export_config.export_path = Some( folder_path.unwrap() );
                }

                let mut path_string = if self.export_config.export_path.is_none()
                {
                    String::new()
                }
                else
                {
                    self.export_config.export_path.clone().unwrap().to_str().unwrap().to_string()
                };

                let text_edit = egui::TextEdit::singleline(&mut path_string)
                .interactive(false);

                ui.add( text_edit);
            });

            let export_button = egui::Button::new("Export project");

            if ui.add_enabled(self.export_config.valid_to_export(), export_button).clicked()
            {
                studio_context.request_compile();
                studio_context.request_export_project(self.export_config.clone());
            }
        });
    }

    fn check_for_available_export_platforms(&mut self)
    {
        let executable_path = std::env::current_exe();
        let runtimes_path = executable_path.unwrap().parent().unwrap().join("resources").join("runtime");

        if !runtimes_path.exists()
        {
            panic!("Runtime doesn't exist");
        }

         let read_directory = std::fs::read_dir(runtimes_path).unwrap();

         for directory_entry in read_directory
         {
             if directory_entry.is_err()
             {
                 continue;
             }

             let directory_entry = directory_entry.unwrap();
             let directory_name = directory_entry.file_name().into_string().unwrap();

             if directory_name.contains("linux") && directory_name.contains("x86_64")
             {
                 self.export_platforms_available.insert(String::from("linux_x86_64"), directory_entry.path());
             }
         }
    }
}
