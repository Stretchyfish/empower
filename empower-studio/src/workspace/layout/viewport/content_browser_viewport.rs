use std::{any::Any, collections::{HashMap, VecDeque}, fs, path::PathBuf};

use crate::{actions::Action, project::{Asset, AssetId, AssetKind, Project, ProjectState}};

use super::Viewport;

const THUMBNAIL_SIZE: egui::Vec2 = egui::Vec2 { x: 100.0, y: 100.0 };
const ELEMENT_SPACING: f32 = 10.0;

pub struct ContentBrowserViewport
{
    known_project_directory: PathBuf, // This is used to detect if the project directory change
    current_directory: Option<PathBuf>,
    directory_distory: VecDeque<PathBuf>,
    selected_asset: Option<PathBuf>,
    show_quick_feature_window: bool,
    quick_feature_window_position_when_activated: egui::Pos2,
    renaming_file: Option<ViewportRenameState>,
}


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
            show_quick_feature_window: false,
            quick_feature_window_position_when_activated: egui::Pos2::new(0.0, 0.0),
            renaming_file: None,
          } ) // @TODO, set this up properly!
    }

    fn name(&self) -> &'static str {
        "content browser viewport"
    }

    fn show(&mut self, ui: &mut egui::Ui, project: &mut Project, viewport_name: &String, action_queue: &mut Vec<Action>) {

        let project_directory = match &project.state // @TODO, find a better way!
        {
            ProjectState::Undefined => panic!("Entered an impossible state"),
            ProjectState::Temporary(path_buf) => path_buf.clone(),
            ProjectState::Saved(path_buf) => path_buf.clone(),
            // ProjectState::Temporary(path_buf) => path_buf.clone().join("assets"),
            // ProjectState::Saved(path_buf) => path_buf.clone().join("assets"),
        };

        if project_directory != self.known_project_directory // @TODO, find a better way!
        {
            self.known_project_directory = project_directory.clone();
            self.current_directory = Some( project_directory.join("assets") );
        }

        let user_clicked_enter = ui.input(|i| i.key_pressed(egui::Key::Enter));
        if user_clicked_enter && self.renaming_file.is_some()
        {
            let rename_file_state = self.renaming_file.as_ref().unwrap();

            let original_path = rename_file_state.path.clone();
            let mut original_path_parrent_directory = rename_file_state.path.clone();
            original_path_parrent_directory.pop(); 
            let new_path = original_path_parrent_directory.join(rename_file_state.potential_new_name.clone());
            
            action_queue.push( Action::RenameFile { original_path, new_path } );
            self.renaming_file = None;
        }
       
        let user_clicked_esp = ui.input(|i| i.key_pressed(egui::Key::Escape));

        if user_clicked_esp && self.renaming_file.is_some()
        {
            self.renaming_file = None;
        }

        let user_left_clicked = ui.input(|i| i.pointer.primary_clicked());
        if user_left_clicked && self.selected_asset.is_some() // If the user clicks on a new asset, is should just clickly get removed and then added again
        // @TODO, consider changing this
        {
            self.selected_asset = None;
            self.renaming_file = None;
        }
        
        ui.horizontal(|ui|
        {
            if ui.button("import asset").clicked()
            {
                // if project.state == ProjectState::Temporary()
                // {
                //     action_queue.push(Action::SaveProject);
                //     return;
                // }
        
                let file_path = rfd::FileDialog::new() // @TODO, consider if this should be in the project struct instead of the viewport?
                                                    .set_title("Import asset") // @TODO, this should probably be in the action of import asset
                                                    .pick_file();

                if file_path.is_none()
                {
                    panic!("Failed to get file path in import asset!"); // @TODO, in the future, handle this error properly!
                }

                action_queue.push( Action::ImportAsset { path: file_path.unwrap() });
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

        ui.horizontal(|ui|
        {

            let mut project_path = match &project.state
            {
                ProjectState::Undefined => todo!(),
                ProjectState::Temporary(path_buf) => path_buf.clone(),
                ProjectState::Saved(path_buf) => path_buf.clone(),
            };
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

        ui.separator();

        egui::panel::SidePanel::left("something").show_inside(ui, |ui|
        {
            ui.label("test");
        });

        egui::panel::CentralPanel::default().show_inside(ui, |ui|
        {
            
                egui::ScrollArea::vertical().show(ui, |ui|
                {
                    let available_width = ui.available_width();
                    let max_elements_per_row = ((available_width + ELEMENT_SPACING) / (THUMBNAIL_SIZE.x + ELEMENT_SPACING)).floor() as i32;

                    self.show_content_browser_elements(ui, project, max_elements_per_row, action_queue);
                });
        });
    }
}

impl ContentBrowserViewport
{
    fn show_content_browser_elements(&mut self, ui: &mut egui::Ui, project: &Project, max_elements_per_row: i32, action_queue: &mut Vec<Action>)
    {
        let pane_rect = ui.max_rect(); // @TODO, combine this into a function
        if ui.rect_contains_pointer(pane_rect) && ui.input(|i| i.pointer.secondary_clicked())
        {
            self.show_quick_feature_window = !self.show_quick_feature_window;

            if self.show_quick_feature_window
            {
                self.quick_feature_window_position_when_activated = ui.input(|i| i.pointer.hover_pos()).unwrap_or(egui::Pos2 {x: 0.0, y: 0.0});
            }
        }
        
        if self.show_quick_feature_window
        {
            self.show_quick_feature_window(ui, action_queue);
        }

        let directory_entries = fs::read_dir(self.current_directory.clone().unwrap());
        let directory_entries = match directory_entries
        {
            Ok( entries ) => entries,
            Err( error ) => 
            {
                println!("Directory location: {:?}", self.current_directory.clone().unwrap());
                println!("Error in reading content browser directory: {}", error.kind().to_string());
                println!("Error expanded: {}", error.to_string());
                return;
            }
        };

        let mut an_asset_was_selected = false;

        ui.horizontal_wrapped(|ui|
        {
            for entry in directory_entries.flatten()
            {
                if entry.path().is_dir()
                {
                    self.draw_directory_asset(ui, &entry.path());
                    continue;
                }

                self.draw_file_asset(ui, &entry.path());
            }
            
        });
    }

    fn draw_directory_asset(&mut self, ui: &mut egui::Ui, asset_path: &PathBuf)
    {
        let mut is_asset_selected = false;
        if self.selected_asset.is_some()
        {
            is_asset_selected = *self.selected_asset.as_ref().unwrap() == *asset_path;
        }

        let directory_name = asset_path.file_name().unwrap().to_string_lossy().to_string();
        
        ui.vertical(|ui|
        {
            ui.allocate_ui_with_layout(
                                    egui::Vec2::new(70.0, 0.0), 
                                    egui::Layout::top_down(egui::Align::Center),
                                    |ui|
            {
                
            let icon = String::from("📁");

            let selectable_label = egui::Button::selectable(is_asset_selected, egui::RichText::new(icon.clone()).font(egui::FontId::proportional(70.0)));

            let selectable_label_response = ui.add(selectable_label).on_hover_text(directory_name.clone());

            // @TODO, this whole approach to renaming doesn't scale
            if self.renaming_file.is_some() // @TODO, maybe instead of renaming file it should be renaming asset?
            {
                let rename_file_state = self.renaming_file.as_mut().unwrap();
                if rename_file_state.path == *asset_path
                {
                    ui.text_edit_singleline(&mut rename_file_state.potential_new_name)
                    .request_focus();
                    return;
                }
            }

            if selectable_label_response.clicked()
            {
                self.selected_asset = Some( asset_path.clone() );
            }

            if selectable_label_response.double_clicked()
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

            let mut short_directory_name = String::new();

            for (index, character) in directory_name.char_indices()
            {
                if index > 11
                {
                    short_directory_name.push_str("...");
                    break;
                }

                short_directory_name.push(character);
            }

            ui.label(short_directory_name);
            });
        });
    }

    fn draw_file_asset(&mut self, ui: &mut egui::Ui, asset_path: &PathBuf)
    {
        let mut is_asset_selected = false;
        if self.selected_asset.is_some()
        {
            is_asset_selected = *self.selected_asset.as_ref().unwrap() == *asset_path;
        }

        let file_name = asset_path.file_name().unwrap().to_string_lossy();

        ui.vertical(|ui|
        {
            ui.allocate_ui_with_layout(
                                        egui::Vec2::new(70.0, 0.0), 
                                        egui::Layout::top_down(egui::Align::Center),
                                        |ui|
        {
            let mut icon = String::from("📃");

            if file_name.ends_with(".png")
            {
                icon = String::from("📷");
            }

            if ui.selectable_label(is_asset_selected, egui::RichText::new(icon).font(egui::FontId::proportional(70.0))).on_hover_text(file_name.clone()).clicked()
            {
                self.selected_asset = Some( asset_path.clone() );
            }

            // @TODO, this whole approach to renaming doesn't scale
            if self.renaming_file.is_some()
            {
                let rename_file_state = self.renaming_file.as_mut().unwrap();
                if rename_file_state.path == *asset_path
                {
                    ui.text_edit_singleline(&mut rename_file_state.potential_new_name)
                    .request_focus();
                    return;
                }
            }

            let mut short_file_name = String::new();

            for (index, character) in file_name.char_indices()
            {
                if index > 11
                {
                    short_file_name.push_str("...");
                    break;
                }

                short_file_name.push(character);
            }

            ui.label(short_file_name);
                                        
        });

        });

    }

    fn show_quick_feature_window(&mut self, ui: &mut egui::Ui, action_queue: &mut Vec<Action>)
    {
        egui::Window::new("")
        .current_pos(egui::Pos2 {
                                x: self.quick_feature_window_position_when_activated.x - 100.0, 
                                y: self.quick_feature_window_position_when_activated.y - 15.0
                                })
        .min_size(egui::Vec2 {x: 200.0, y: 200.0})
        .max_size(egui::Vec2 {x: 200.0, y: 200.0})
        .title_bar(false)
        .show(ui.ctx(), |ui|
        {
            if ui.add(egui::Button::new("create file").min_size(egui::Vec2 {x: 190.0, y: 20.0})).clicked() 
            {
                let new_asset_path = self.current_directory.as_ref().unwrap().clone().join("unamed.txt");
                action_queue.push( Action::CreateFile { path: new_asset_path.clone() } );
                self.renaming_file = Some( ViewportRenameState::new( &new_asset_path ) );
                self.show_quick_feature_window = false;
            };
            
            if ui.add(egui::Button::new("create folder").min_size(egui::Vec2 {x: 190.0, y: 20.0})).clicked() 
            {
                let new_asset_path = self.current_directory.as_ref().unwrap().clone().join("unamed");
                action_queue.push( Action::CreateFolder { path: new_asset_path.clone() } );
                self.renaming_file = Some( ViewportRenameState::new( &new_asset_path ) );
                self.show_quick_feature_window = false;
            };

            if self.selected_asset.is_some()
            {
                if ui.add(egui::Button::new("rename file").min_size(egui::Vec2 {x: 190.0, y: 20.0})).clicked() 
                {
                    // @TODO, more dangerous unwraps without checks here
                    self.renaming_file = Some( ViewportRenameState::new( self.selected_asset.as_ref().unwrap() ) );
                    self.show_quick_feature_window = false;
                };
            }
        });
    }
}

enum QuickMenuBehavior
{
    CreateFile,
    CreateFolder,
    RenameFile,
}

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
