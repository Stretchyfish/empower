use std::{fs, path::PathBuf};
use crate::studio_context::project::Project;
use std::collections::BTreeMap;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct ExportSettings
{
    pub show: bool,
    export_name: String,
    export_path: PathBuf,

    #[serde(skip, default)]
    target_platform: Option<String>, 

    #[serde(skip, default)]
    export_platforms_available: BTreeMap<String, PathBuf>,

    #[serde(skip, default)]
    checked_atleast_once_for_runtimes: bool,
}

impl ExportSettings
{
    pub fn new() -> Self
    {
        Self
        {
            show: false,
            export_name: String::new(),
            target_platform: None,
            export_path: PathBuf::new(),

            export_platforms_available: BTreeMap::new(),
            checked_atleast_once_for_runtimes: false,
        }
    }

    pub fn toggle_show(&mut self)
    {
        self.show = !self.show;
    }

    pub fn show(&mut self, ctx: &egui::Context, project: &mut Project)
    {
        if !self.show { return; }

        if !self.checked_atleast_once_for_runtimes
        {
            let executable_path = std::env::current_exe();
            let runtimes_path = executable_path.unwrap().parent().unwrap().join("resources").join("runtime");

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
            
            self.checked_atleast_once_for_runtimes = true;
        }

        egui::Window::new("Export Settings")
        .collapsible(true)
        .resizable(true)
        .auto_sized()
        .open(&mut self.show)
        .show(ctx, |ui| 
        {
            ui.horizontal(|ui|
            {
                ui.label("Exported name: ");
                let text_edit = egui::TextEdit::singleline(&mut self.export_name);

                ui.add( text_edit);
            });

            ui.horizontal(|ui|
            {
                ui.label("Available platforms: ");

                let desired_platform_name = match self.target_platform.clone()
                {
                    Some( name ) => name,
                    None => String::from("undefined"),
                };
                
                ui.menu_button(desired_platform_name, |ui|
                {
                    for (platform_name, _) in &self.export_platforms_available
                    {
                        if ui.button(platform_name).clicked()
                        {
                            self.target_platform = Some( platform_name.clone() );
                        }
                    }
                });

            });

            if self.export_platforms_available.is_empty()
            {
                let info_text = egui::RichText::new("No platform runtime could be found, exporting currently not available").color(egui::Color32::RED);
                ui.label(info_text);
            }

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

                    self.export_path = folder_path.unwrap();
                }
                
                let mut path_string = self.export_path.clone().to_str().unwrap().to_string();
                let text_edit = egui::TextEdit::singleline(&mut path_string)
                .interactive(false);

                ui.add( text_edit);
            });

            let export_button = egui::Button::new("Export project");

            if ui.add_enabled(self.target_platform.is_some(), export_button).clicked()
            {
                export_behavior(project, self.export_name.clone(), self.export_path.clone(), self.export_platforms_available.get(&self.target_platform.clone().unwrap()).unwrap().clone());
            }
        });
    }
}

fn export_behavior(project: &mut Project, export_name: String, location: PathBuf, platform_path: PathBuf)
{
    if !platform_path.exists()
    {
        panic!("Runtime when exporting doesn't exist: {}", platform_path.to_string_lossy().to_string());
    }

    // @TODO, add behavior for windows
 
    let export_directory = location.join(export_name.clone());

    let copy_options = fs_extra::dir::CopyOptions::new().copy_inside(true);
    let copy_runtime_result = fs_extra::dir::copy(platform_path.clone(), &export_directory, &copy_options);

    match copy_runtime_result
    {
        Ok(_) => {},
        Err( error ) =>
        {
            println!("Error when copying runtime: {}", error.to_string());
        },
    }

    let _ = fs::rename(export_directory.join(platform_path.file_name().unwrap()), export_directory.join(export_name.clone()));

    let new_runtime_location_before_name_change = if cfg!(target_os = "windows")
    {
        export_directory.join("empower-application.exe")
    }
    else
    {
        export_directory.join("empower-application")
    };

    let new_runtime_location_after_name_change = if cfg!(target_os = "windows")
    {
         export_directory.join(format!("{}.exe", export_name.clone()))
    }
    else
    {
         export_directory.join(export_name.clone())
    };

    let rename_exutable_result = fs::rename(new_runtime_location_before_name_change, new_runtime_location_after_name_change.clone());

    match rename_exutable_result
    {
        Ok(_) => {},
        Err( error ) =>
        {
            println!("Failed to rename exported executable due to error : {}", error.kind().to_string());
        },
    }
    
    let export_name = "graph.json";
    project.graph_editor.node_graph.save(export_directory.join(export_name));
}

