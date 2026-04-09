use std::{fmt, fs::{self, exists}, path::PathBuf};
use crate::studio_context::project::Project;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct ExportSettings
{
    pub show: bool,
    export_name: String,
    target_platform: Platforms, 
    export_path: PathBuf,
}

impl ExportSettings
{
    pub fn new() -> Self
    {
        Self
        {
            show: false,
            export_name: String::new(),
            target_platform: Platforms::Undefined,
            export_path: PathBuf::new(),
        }
    }

    pub fn toggle_show(&mut self)
    {
        self.show = !self.show;
    }

    pub fn show(&mut self, ctx: &egui::Context, project: &mut Project)
    {
        if !self.show { return; }

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
                // .char_limit(5)
                // .text_color(text_edit_color);
                // .background_color(text_background_color);

                ui.add( text_edit);
            });

            ui.horizontal(|ui|
            {
                ui.label("Target platform: ");
                ui.menu_button(self.target_platform.to_string(), |ui|
                {
                    if ui.button("Linux").clicked()
                    {
                        self.target_platform = Platforms::Linux;
                    }
                    if ui.button("Windows").clicked()
                    {
                        self.target_platform = Platforms::Windows;
                    }
                    if ui.button("Mac").clicked()
                    {
                        self.target_platform = Platforms::Mac;
                    }
                    if ui.button("Web").clicked()
                    {
                        self.target_platform = Platforms::Web;
                    }
                });
            });

            let info_text = egui::RichText::new("Platform selection currently not working, will default to current platform").color(egui::Color32::RED);
            ui.label(info_text);

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

            if ui.button("Export Project").clicked()
            {
                export_behavior(project, self.export_name.clone(), self.export_path.clone());

                
            }
        });
    }
}

#[derive(Clone, PartialEq, Eq, Debug, serde::Serialize, serde::Deserialize)]
enum Platforms
{
    Undefined,
    Linux,
    Mac,
    Windows,
    Web,
}

impl fmt::Display for Platforms
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result 
    {
        write!(f, "{:?}", self)
    }
}

fn export_behavior(project: &mut Project, export_name: String, location: PathBuf)
{

    let executable_path = std::env::current_exe();
    // println!("Application path: {}", application_path.unwrap().parent().unwrap().to_string_lossy().to_string());

    // @TODO, update this to work on windows
    let runtime_path = executable_path.unwrap().parent().unwrap().join("resources").join("empower-application");

    match exists(runtime_path.clone())
    {
        Ok(_) => {},
        Err(_) =>
        {
            {
                panic!("Runtime when exporting doesn't exist: {}", runtime_path.to_string_lossy().to_string());
            }
        },
    }
 
    let export_directory = location.join(export_name.clone());

    let copy_options = fs_extra::dir::CopyOptions::new().copy_inside(true);
    let copy_runtime_result = fs_extra::dir::copy(runtime_path.clone(), &export_directory, &copy_options);

    match copy_runtime_result
    {
        Ok(_) => {},
        Err( error ) =>
        {
            println!("Error when copying runtime: {}", error.to_string());
        },
    }

    fs::rename(export_directory.join("empower-application"), export_directory.join(export_name.clone()));
    
    let export_name = export_name + ".json";
    project.graph_editor.node_graph.save(export_directory.join(export_name));

    // let res = fs::copy(runtime_path.clone(), export_directory);

    // match res
    // {
    //     Ok(_) => todo!(),
    //     Err( error ) =>
    //     {
    //         println!("Error when copying runtime: {}", error.kind().to_string());
    //     },
    // }

    
}
