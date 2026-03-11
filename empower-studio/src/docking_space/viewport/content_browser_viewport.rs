use std::{collections::VecDeque, fs, path::PathBuf};

use crate::{studio_context::{StudioContext, project::Project}, user_inputs::UserInputs};

use super::Viewport;

// use super::DraggedAsset;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct ContentBrowserViewport
{
    known_project_directory: PathBuf, // This is used to detect if the project directory change
    current_directory: Option<PathBuf>,
    directory_distory: VecDeque<PathBuf>,
    selected_asset: Option<PathBuf>,
    quick_menu: Option<egui::Pos2>,
    renaming_file: Option<ViewportRenameState>,
}

#[typetag::serde]
impl Viewport for ContentBrowserViewport
{
    fn new() -> Box<dyn Viewport> 
    where
        Self: Sized {

        Box::new( Self { 
            known_project_directory: PathBuf::new(),
            current_directory: None, 
            directory_distory: VecDeque::new(),
            selected_asset: None, 

            quick_menu: None,
            renaming_file: None,
          } ) // @TODO, set this up properly!
    }

    fn clone_box(&self) -> Box<dyn Viewport>
    {
        Box::new( self.clone() )
    }

    fn name(&self) -> &'static str {
        "content browser viewport"
    }

    fn show(&mut self, ui: &mut egui::Ui, studio_context: &mut StudioContext, _: &String, user_inputs: &UserInputs) {

        let project = studio_context.get_project_mut();
        let project_directory = &project.location;

        if *project_directory != self.known_project_directory // @TODO, find a better way!
        {
            self.known_project_directory = project_directory.clone();
            self.current_directory = Some( project_directory.join("assets") );
        }

        self.show_asset_import_and_directory_navigation(ui, project);
        self.show_breadcrum_path(ui, project);
        ui.separator();
        self.show_content_browser_category_panel(ui);
        self.show_content_browser_elements_panel(ui, project, user_inputs);
    }
}

impl ContentBrowserViewport
{
    pub fn process_user_inputs(&mut self, ui: &egui::Ui, user_inputs: &UserInputs, project: &mut Project)
    {
        if !ui.max_rect().contains(user_inputs.mouse_position) { return; }
        
        if user_inputs.clicked_enter && self.renaming_file.is_some()
        {
            let rename_file_state = self.renaming_file.as_ref().unwrap();

            let original_path = rename_file_state.path.clone();
            let mut original_path_parrent_directory = rename_file_state.path.clone();
            original_path_parrent_directory.pop(); 
            let new_path = original_path_parrent_directory.join(rename_file_state.potential_new_name.clone());
            
            project.rename_file(original_path, new_path);
            self.renaming_file = None;
            return;
        }
       
        if ( user_inputs.clicked_esp || user_inputs.clicked_primary_mouse_button || user_inputs.clicked_secondary_mouse_button) && self.renaming_file.is_some()
        {
            self.renaming_file = None;
            return;
        }

        if user_inputs.clicked_primary_mouse_button && self.selected_asset.is_some() // If the user clicks on a new asset, is should just clickly get removed and then added again
        {
            self.selected_asset = None;
            self.renaming_file = None;
            return;
        }

        if user_inputs.clicked_secondary_mouse_button
        {
            if self.quick_menu.is_none()
            {
                self.quick_menu = Some( user_inputs.mouse_position.clone() );
                return;
            }

            self.quick_menu = None;
        }
    }

    pub fn show_asset_import_and_directory_navigation(&mut self, ui: &mut egui::Ui, project: &mut Project)
    {
        ui.horizontal(|ui|
        {
            if ui.button("import asset").clicked()
            {
                let file_path = rfd::FileDialog::new() // @TODO, consider if this should be in the project struct instead of the viewport?
                                                    .set_title("Import asset") // @TODO, this should probably be in the action of import asset
                                                    .pick_file();

                if file_path.is_some()
                {
                    project.import_asset(&file_path.unwrap());
                }
            }

            if ui.button("👈").clicked()
            {
                // @TODO, find a better approach
                if self.directory_distory.len() > 10
                {
                    self.directory_distory.pop_front();
                }
                self.directory_distory.push_back(self.current_directory.as_ref().unwrap().clone());

                // @TODO, this whole thing is very dangerous, find a better way
                self.current_directory.as_mut().unwrap().pop();
            }

            if !self.directory_distory.is_empty()
            {
                if ui.button("👉").clicked()
                {
                    self.current_directory = self.directory_distory.pop_front();
                }
            }
        });
    }

    pub fn show_breadcrum_path(&mut self, ui: &mut egui::Ui, project: &mut Project)
    {
        ui.horizontal(|ui|
        {
            let mut project_path = project.location.clone();
            project_path.pop();

            // @TODO, move all this breadcrum path stuff into its own function

            let mut clicked_breadcrum_path_button = None;

            let mut accumelating_path = PathBuf::from(project_path.clone());

            let binding = self.current_directory.clone().unwrap();
            let relative_path = match binding.strip_prefix(project_path)
            {
                Ok( path ) => path,
                Err( error ) => panic!("Unable to get a relative path to project, directories does not match, error: {}", error.to_string()),
            };

            for path_component in relative_path.components().filter_map(|comp| match comp
                {
                    std::path::Component::Normal(n) => Some(n),
                    _ => panic!("incompatible path"),
                })
            {
                accumelating_path.push(path_component);

                let breadcrum_path_button = egui::Button::new(path_component.to_string_lossy()).frame(false);

                let breadcrum_path_response = ui.add(breadcrum_path_button);

                if breadcrum_path_response.clicked()
                {
                    clicked_breadcrum_path_button = Some( accumelating_path.clone() );
                }

                ui.label("/");
            }

            if clicked_breadcrum_path_button.is_some()
            {
                self.current_directory = Some( clicked_breadcrum_path_button.as_ref().unwrap().clone() );
            }
        });
    }

    pub fn show_content_browser_category_panel(&mut self, ui: &mut egui::Ui)
    {
        egui::panel::SidePanel::left("something").show_inside(ui, |ui|
        {
            if ui.button("assets").clicked()
            {
                self.current_directory = Some( self.known_project_directory.join("assets") );
            }
        });
    }

    pub fn show_content_browser_elements_panel(&mut self, ui: &mut egui::Ui, project: &mut Project, user_inputs: &UserInputs)
    {
        if self.current_directory.is_none()
        {
            return;
        }

        let directory_entries = fs::read_dir(self.current_directory.clone().unwrap());
        let directory_entries = match directory_entries
        {
            Ok( entries ) => entries,
            Err( error ) => 
            {
                match error.kind()
                {
                    std::io::ErrorKind::NotFound => // This error is most likely to happen if the closed the application in a new folder in a temporary project
                    {
                        self.current_directory = Some( project.location.join("assets") );
                        println!("Overwritting current directory in content browser due to impossible access when loading layout");
                        return;
                    }, 
                    _ => { panic!("Error in reading content browser directory: {}", error.kind().to_string()); },
                }
            }
        };
        
        egui::panel::CentralPanel::default().show_inside(ui, |ui|
        {
            self.process_user_inputs(ui, user_inputs, project);
            
            if self.quick_menu.is_some()
            {
                self.show_quick_feature_window(ui, project, &self.quick_menu.unwrap());
            }
            
            egui::ScrollArea::vertical().show(ui, |ui|
            {
                ui.horizontal_wrapped(|ui|
                {
                    for entry in directory_entries.flatten()
                    {
                        self.show_asset(ui, &entry.path());
                    }
                });
            });
        });

        if !ui.max_rect().contains(user_inputs.mouse_position)
        {
            return;
        }

    
    }

    fn show_asset(&mut self, ui: &mut egui::Ui, asset_path: &PathBuf)
    {
        let mut is_asset_selected = false;
        if self.selected_asset.is_some()
        {
            is_asset_selected = *self.selected_asset.as_ref().unwrap() == *asset_path;
        }

        let mut file_is_being_renamed = false;
        if self.renaming_file.is_some()
        {
            file_is_being_renamed = self.renaming_file.as_ref().unwrap().path == *asset_path;
        }

        let asset_is_a_directory = asset_path.is_dir();

        let asset_name = asset_path.file_name().unwrap().to_string_lossy().to_string();
       
        ui.vertical(|ui|
        {
            ui.allocate_ui_with_layout(
                                        egui::Vec2::new(70.0, 0.0), 
                                        egui::Layout::top_down(egui::Align::Center),
                                        |ui|
            {
                let mut asset_icon = String::from("📃");

                if asset_is_a_directory
                {
                    asset_icon = String::from("📁");
                }

                if asset_name.ends_with(".png")
                {
                    asset_icon = String::from("📷");
                }

                let selectable_label = egui::Button::selectable(is_asset_selected, egui::RichText::new(asset_icon.clone()).font(egui::FontId::proportional(70.0))).sense(egui::Sense::click_and_drag());
                let selectable_asset_response = ui.add(selectable_label).on_hover_text(asset_name.clone());

                if selectable_asset_response.drag_started()
                {
                    // action_queue.push( Action::BeginDraggingAsset { path: asset_path.clone() });
                    // selectable_asset_response.dnd_set_drag_payload( DraggedAsset { path: asset_path.clone() } );
                }

                if selectable_asset_response.drag_stopped()
                {
                    // action_queue.push( Action::StopDraggingAsset );
                }

                if selectable_asset_response.clicked()
                {
                    self.selected_asset = Some( asset_path.clone() );
                }

                if selectable_asset_response.double_clicked()
                {
                    if asset_is_a_directory
                    {
                        
                        // @TODO, find a better apparoach
                        if self.directory_distory.len() > 10
                        {
                            self.directory_distory.pop_front();
                        }
                        self.directory_distory.push_back(self.current_directory.as_ref().unwrap().clone());

                        self.current_directory = Some( asset_path.clone() );
                        self.selected_asset = None;
                    }

                }

                if file_is_being_renamed // @TODO, This is a somewhat dangerous (should be save, but still)
                {
                    ui.text_edit_singleline(&mut self.renaming_file.as_mut().unwrap().potential_new_name)
                    .request_focus();
                    return;
                }

                let potentially_shortened_asset_name = shorten_text(asset_name, 11);
                ui.label(potentially_shortened_asset_name);
                
            })
        });
    }
    
    fn show_quick_feature_window(&mut self, ui: &mut egui::Ui, project: &mut Project, mouse_position_when_activated: &egui::Pos2)
    {
        egui::Window::new("")
        .current_pos(egui::Pos2 {
                                x: mouse_position_when_activated.x - 100.0, 
                                y: mouse_position_when_activated.y - 15.0
                                })
        .min_size(egui::Vec2 {x: 200.0, y: 200.0})
        .max_size(egui::Vec2 {x: 200.0, y: 200.0})
        .title_bar(false)
        .show(ui.ctx(), |ui|
        {
            if ui.add(egui::Button::new("create file").min_size(egui::Vec2 {x: 190.0, y: 20.0})).clicked() 
            {
                let new_asset_path = self.current_directory.as_ref().unwrap().clone().join("unamed.txt");

                project.create_file(&new_asset_path);
                self.renaming_file = Some( ViewportRenameState::new( &new_asset_path ) );
                self.quick_menu = None;
            };
            
            if ui.add(egui::Button::new("create folder").min_size(egui::Vec2 {x: 190.0, y: 20.0})).clicked() 
            {
                let new_asset_path = self.current_directory.as_ref().unwrap().clone().join("unamed");
                project.create_folder(&new_asset_path);
                self.renaming_file = Some( ViewportRenameState::new( &new_asset_path ) );
                self.quick_menu = None;
            };

            if self.selected_asset.is_some()
            {
                if ui.add(egui::Button::new("rename file").min_size(egui::Vec2 {x: 190.0, y: 20.0})).clicked() 
                {
                    // @TODO, more dangerous unwraps without checks here
                    self.renaming_file = Some( ViewportRenameState::new( self.selected_asset.as_ref().unwrap() ) );
                    self.quick_menu = None;
                };
            }
        });
    }
}

fn shorten_text(mut text: String, max_letters: usize) -> String // @TODO, make this its own tool
{
    if text.len() <= max_letters
    {
        return text;
    }
    
    text.truncate(max_letters);
    text.push_str("...");

    text
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct ViewportRenameState
{
    potential_new_name: String,
    path: PathBuf,
}

impl ViewportRenameState
{
    pub fn new(path: &PathBuf) -> Self
    {
        Self
        {
            potential_new_name: path.file_name().unwrap().to_string_lossy().to_string(),
            path: path.clone(), 
        }
    }
}

// #[derive(Clone, serde::Serialize, serde::Deserialize)]
// struct QuickMenu
// {
//     mouse_position_when_activated: egui::Pos2,
// }

// impl QuickMenu
// {
//     pub fn new(mouse_position: &egui::Pos2) -> Self
//     {
//         Self
//         {
//             mouse_position_when_activated: *mouse_position
//         }
//     }

//     pub fn show(&self, ui: &mut egui::Ui)
//     {
//         egui::Window::new("")
//         .current_pos(egui::Pos2 {
//                                 x: self.mouse_position_when_activated.x - 100.0, 
//                                 y: self.mouse_position_when_activated.y - 15.0
//                                 })
//         .min_size(egui::Vec2 {x: 200.0, y: 200.0})
//         .max_size(egui::Vec2 {x: 200.0, y: 200.0})
//         .title_bar(false)
//         .show(ui.ctx(), |ui|
//         {
//             if ui.add(egui::Button::new("create file").min_size(egui::Vec2 {x: 190.0, y: 20.0})).clicked() 
//             {
//                 let new_asset_path = self.current_directory.as_ref().unwrap().clone().join("unamed.txt");

//                 project.create_file(&new_asset_path);
//                 self.renaming_file = Some( ViewportRenameState::new( &new_asset_path ) );
//                 self.show_quick_feature_window = false;
//             };
        
//             if ui.add(egui::Button::new("create folder").min_size(egui::Vec2 {x: 190.0, y: 20.0})).clicked() 
//             {
//                 let new_asset_path = self.current_directory.as_ref().unwrap().clone().join("unamed");
//                 project.create_folder(&new_asset_path);
//                 self.renaming_file = Some( ViewportRenameState::new( &new_asset_path ) );
//                 self.show_quick_feature_window = false;
//             };

//             if self.selected_asset.is_some()
//             {
//                 if ui.add(egui::Button::new("rename file").min_size(egui::Vec2 {x: 190.0, y: 20.0})).clicked() 
//                 {
//                     // @TODO, more dangerous unwraps without checks here
//                     self.renaming_file = Some( ViewportRenameState::new( self.selected_asset.as_ref().unwrap() ) );
//                     self.show_quick_feature_window = false;
//                 };
//             }
//         });
//     }
// }
