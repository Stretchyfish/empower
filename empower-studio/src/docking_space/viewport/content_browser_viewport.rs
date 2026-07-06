use std::{fs, path::PathBuf};

use crate::{docking_space::{Viewport, viewport::GraphViewport}, studio_context::StudioContext, user_inputs::UserInputs};
use empower_engine::assets::AssetKind;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct ContentBrowserViewport
{
    #[serde(skip)]
    known_project_directory: PathBuf, // This is used to detect if the project directory change

    #[serde(skip)]
    current_directory: Option<PathBuf>,

    #[serde(skip)]
    selected_asset: Option<PathBuf>,

    #[serde(skip)]
    quick_menu: Option<egui::Pos2>,

    #[serde(skip)]
    renaming_file: Option<ViewportRenameState>,
}

impl ContentBrowserViewport
{
    pub fn new() -> Self
    {
        Self {
            known_project_directory: PathBuf::new(),
            current_directory: None, 
            selected_asset: None, 
            quick_menu: None,

            renaming_file: None,
        }
    }
}

pub fn show(content_browser_viewport: &mut ContentBrowserViewport, ui: &mut egui::Ui, studio_context: &mut StudioContext, _: &String, user_inputs: &UserInputs)
{
    if ui.max_rect().contains(user_inputs.mouse_position)
    {
        content_browser_viewport.process_user_inputs(user_inputs, studio_context);
    }

    {
        let project_directory = studio_context.get_project().location.clone();

        if *project_directory != content_browser_viewport.known_project_directory // @TODO, find a better way! This will probably be naturally fixes when using an ViewportConfigurations
        {
            content_browser_viewport.known_project_directory = project_directory.clone();
            content_browser_viewport.current_directory = Some( project_directory.join("assets") );
        }

        content_browser_viewport.show_asset_import_and_directory_navigation(ui, studio_context);
        content_browser_viewport.show_breadcrum_path(ui);
        ui.separator();
    }

    content_browser_viewport.show_content_browser_elements_panel(ui, studio_context, user_inputs);
}

impl ContentBrowserViewport
{
    pub fn show_asset_import_and_directory_navigation(&mut self, ui: &mut egui::Ui, studio_context: &mut StudioContext)
    {
        ui.horizontal(|ui|
        {
            if ui.button("import asset").clicked()
            {
                let file_path = rfd::FileDialog::new() // @TODO, consider if this should be in the project struct instead of the viewport?
                                                    .set_title("Import asset") // @TODO, this should probably be in the action of import asset
                                                    .pick_file();

                // let file_path: Option<PathBuf> = None; // @TODO, fix this!

                if file_path.is_some()
                {
                    let import_location = self.current_directory.as_ref().unwrap().clone().join(file_path.as_ref().unwrap().file_name().unwrap());
                    let file_copy_result = fs::copy(file_path.as_ref().unwrap(), &import_location);

                    match file_copy_result
                    {
                        Ok(_) => {},
                        Err( error ) => println!("Error when copying imported file: {}", error.kind().to_string()),
                    }

                    studio_context.get_project_mut().assets.import_asset(&import_location);
                }
            }

            if ui.button("👈").clicked()
            {
                // // @TODO, find a better approach
                // if self.directory_distory.len() > 10
                // {
                //     self.directory_distory.pop_front();
                // }
                // self.directory_distory.push_back(self.current_directory.as_ref().unwrap().clone());

                // // @TODO, this whole thing is very dangerous, find a better way
                // self.current_directory.as_mut().unwrap().pop();
            }

            if ui.button("👉").clicked()
            {
                // self.current_directory = self.directory_distory.pop_front();
            }
            // if !self.directory_distory.is_empty()
            // {
            // }
        });
    }

    pub fn show_breadcrum_path(&mut self, ui: &mut egui::Ui)
    {
        ui.horizontal(|ui|
        {
            let mut project_path = self.known_project_directory.clone();
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

    pub fn show_content_browser_elements_panel(&mut self, ui: &mut egui::Ui, studio_context: &mut StudioContext, user_inputs: &UserInputs)
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
                        self.current_directory = Some( self.known_project_directory.join("assets") );
                        println!("Overwritting current directory in content browser due to impossible access when loading layout");
                        return;
                    }, 
                    _ => { panic!("Error in moving file: {}", error.kind().to_string()); },
                }
            }
        };
        
        egui::panel::CentralPanel::default().show_inside(ui, |ui|
        {
            if self.quick_menu.is_some()
            {
                self.show_quick_feature_window(ui, studio_context, &self.quick_menu.unwrap());
            }
            
            // self.hovering_asset = None; // This will get set back to the actually hovered asset if the user is still hovering in show_asset
            egui::ScrollArea::vertical().show(ui, |ui|
            {
                ui.horizontal_wrapped(|ui|
                {
                    for entry in directory_entries.flatten()
                    {
                        self.show_asset(ui, &entry.path(), studio_context);
                    }
                });
            });
        });

        if !ui.max_rect().contains(user_inputs.mouse_position)
        {
            return;
        }
    }

    fn show_asset(&mut self, ui: &mut egui::Ui, asset_path: &PathBuf, studio_context: &mut StudioContext)
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
                    // studio_context.request_begin_dragging_asset(asset_path);
                }

                if selectable_asset_response.hovered()
                {
                    // self.hovering_asset = Some( asset_path.clone() );
                }

                if selectable_asset_response.clicked()
                {
                    self.selected_asset = Some( asset_path.clone() );
                }

                if selectable_asset_response.double_clicked()
                {
                    if asset_is_a_directory
                    {
                        
                        // // @TODO, find a better apparoach
                        // if self.directory_distory.len() > 10
                        // {
                        //     self.directory_distory.pop_front();
                        // }
                        // self.directory_distory.push_back(self.current_directory.as_ref().unwrap().clone());

                        self.current_directory = Some( asset_path.clone() );
                        // self.selected_asset = None;
                        return;
                    }

                    let asset_id = studio_context.get_project().assets.path_to_asset_id.get(asset_path);

                    if asset_id.is_none()
                    {
                        panic!("Content browser tried to access an id which is not in the asset"); // @TODO, find a proper way of handling this
                    }

                    let asset_meta = studio_context.get_project().assets.meta.get(asset_id.unwrap()).unwrap();

                    match asset_meta.kind
                    {
                        AssetKind::Graph => { studio_context.request_new_viewport_at_first_docking_leaf( Viewport::Graph { graph_viewport: GraphViewport::new(asset_meta.id) }) },
                        AssetKind::Image => { studio_context.request_new_viewport_at_first_docking_leaf( Viewport::ImageViewer { image_asset_id: asset_meta.id }) },
                    }

                    return;
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

    pub fn process_user_inputs(&mut self, user_inputs: &UserInputs, studio_context: &mut StudioContext)
    {
        let project = studio_context.get_project_mut();
        
        if user_inputs.clicked_secondary_mouse_button
        {
            if self.quick_menu.is_none()
            {
                self.quick_menu = Some( user_inputs.mouse_position.clone() );
                return;
            }

            self.quick_menu = None;
        }

        if user_inputs.clicked_enter && self.renaming_file.is_some()
        {
            let rename_file_state = self.renaming_file.as_ref().unwrap();

            let original_path = rename_file_state.path.clone();
            let mut original_path_parrent_directory = rename_file_state.path.clone();
            original_path_parrent_directory.pop(); 
            let new_path = original_path_parrent_directory.join(rename_file_state.potential_new_name.clone());
            
            project.assets.rename_file(original_path, new_path);

            self.renaming_file = None;
            return;
        }

        // @TODO, update this to be, detect background inside of the viewport instead

        if ( user_inputs.clicked_esp || user_inputs.clicked_primary_mouse_button || user_inputs.clicked_secondary_mouse_button) && self.renaming_file.is_some()
        {
            self.renaming_file = None;
            return;
        }

        if ( user_inputs.clicked_esp || user_inputs.clicked_primary_mouse_button || user_inputs.clicked_secondary_mouse_button) && self.selected_asset.is_some()
        {
            self.selected_asset = None;
            self.quick_menu = None;
            return;
        }



        
    }

    fn show_quick_feature_window(&mut self, ui: &mut egui::Ui, studio_context: &mut StudioContext, mouse_position_when_activated: &egui::Pos2)
    {
        let project = studio_context.get_project_mut();

        let project_path = &project.location; // @TODO, in the future this needs to be handled at runtime
        let assets = &mut project.assets;
        
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
                // let new_asset_path = self.current_directory.as_ref().unwrap().clone().join("unamed.txt");
                // assets.create_file(&new_asset_path);

                // self.renaming_file = Some( ViewportRenameState::new( &new_asset_path ) );
                self.quick_menu = None;
            };
            
            if ui.add(egui::Button::new("create folder").min_size(egui::Vec2 {x: 190.0, y: 20.0})).clicked() 
            {
                let new_asset_path = self.current_directory.as_ref().unwrap().clone().join("unamed");
                assets.create_folder(&new_asset_path);

                self.renaming_file = Some( ViewportRenameState::new( &new_asset_path ) );
                self.quick_menu = None;
            };

            if ui.add(egui::Button::new("create graph").min_size(egui::Vec2 {x: 190.0, y: 20.0})).clicked() 
            {
                let asset_id = assets.create_asset(self.current_directory.as_ref().unwrap(), AssetKind::Graph, None);

                if asset_id.is_some()
                {
                    // self.renaming_file = Some( ViewportRenameState::new( &new_asset_path ) ); // @TODO, add this back, but it needs to use asset id instead of path
                }

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

#[derive(Clone, Serialize, Deserialize)]
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

