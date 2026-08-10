use std::{collections::VecDeque, path::PathBuf};

use crate::{studio_context::StudioContext, user_inputs::UserInputs};
use empower_engine::assets::{ASSET_FOLDER_ASSET_ID, AssetId, AssetKind, AssetMeta};
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct ContentBrowserViewport
{
    #[serde(skip, default = "default_content_browser_directory")]
    current_directory: AssetId,

    #[serde(skip)]
    hovered_asset: Option<AssetId>,

    #[serde(skip)]
    selected_asset: Option<AssetId>,

    #[serde(skip)]
    quick_menu: Option<egui::Pos2>,

    #[serde(skip)]
    renaming_file: Option<ViewportRenameState>,
}

fn default_content_browser_directory() -> AssetId
{
    ASSET_FOLDER_ASSET_ID
}

impl ContentBrowserViewport
{
    pub fn new() -> Self
    {
        Self {
            current_directory: default_content_browser_directory(), 
            hovered_asset: None,
            selected_asset: None, 
            quick_menu: None,

            renaming_file: None,
        }
    }
}

pub fn show(content_browser_viewport: &mut ContentBrowserViewport, ui: &mut egui::Ui, studio_context: &mut StudioContext, _: &String, user_inputs: &UserInputs)
{
    let developer_mode = studio_context.get_settings().developer_mode;

    if ui.max_rect().contains(user_inputs.mouse_position)
    {
        // content_browser_viewport.process_user_inputs(user_inputs, studio_context);
    }

    let mut actions = Vec::new();
    
    content_browser_viewport.show_asset_import_and_directory_navigation(ui, studio_context);
    content_browser_viewport.show_breadcrum_path(ui, studio_context, &mut actions);
    ui.separator();

    content_browser_viewport.show_content_browser_elements_panel(ui, studio_context, user_inputs, &mut actions);

    if developer_mode
    {
        show_content_browser_debug_info(content_browser_viewport, ui);
    }

    content_browser_viewport.process_actions(actions);
}

impl ContentBrowserViewport
{
    pub fn show_asset_import_and_directory_navigation(&self, ui: &mut egui::Ui, studio_context: &mut StudioContext)
    {
        ui.horizontal(|ui|
        {
            if ui.button("import asset").clicked()
            {
                studio_context.request_import_asset();
            }

            if ui.button("👈").clicked()
            {
            }

            if ui.button("👉").clicked()
            {
            }
        });
    }

    pub fn show_breadcrum_path(&self, ui: &mut egui::Ui, studio_context: &StudioContext, actions: &mut Vec<ContentBrowserViewportAction>)
    {
        let meta = &studio_context.get_project().assets.meta;
        
        ui.horizontal(|ui|
        {
            let mut clicked_breadcrum_path_button = None;

            let mut breadcrum_path_assets = VecDeque::new();

            let mut next_meta_to_check = meta.get(&self.current_directory).expect("Tried to access current directory, but it doesn't exist");
            while next_meta_to_check.parent.is_some()
            {
                breadcrum_path_assets.push_front(next_meta_to_check);
                next_meta_to_check = meta.get(&next_meta_to_check.parent.unwrap()).unwrap();
            }
            breadcrum_path_assets.push_front(next_meta_to_check);

            for folder_asset in breadcrum_path_assets 
            {
                let breadcrum_path_button = egui::Button::new(folder_asset.name.clone()).frame(false);

                let breadcrum_path_response = ui.add(breadcrum_path_button);

                if breadcrum_path_response.clicked()
                {
                    clicked_breadcrum_path_button = Some( folder_asset.id );
                }

                ui.label("/");
            }

            if clicked_breadcrum_path_button.is_some()
            {
                actions.push( ContentBrowserViewportAction::ChangeDirectory( clicked_breadcrum_path_button.as_ref().unwrap().clone() ));
            }
        });
    }

    pub fn show_content_browser_elements_panel(&mut self, ui: &mut egui::Ui, studio_context: &mut StudioContext, user_inputs: &UserInputs, actions: &mut Vec<ContentBrowserViewportAction>)
    {
        let meta = &studio_context.get_project().assets.meta;

        egui::panel::CentralPanel::default().show_inside(ui, |ui|
        {
            if self.quick_menu.is_some()
            {
                self.show_quick_feature_window(ui, &self.quick_menu.unwrap());
            }


            // @TODO, need too add a sorting based way of showing elements
            
            // self.hovering_asset = None; // This will get set back to the actually hovered asset if the user is still hovering in show_asset
            let mut hovered_asset = false;
            egui::ScrollArea::vertical().show(ui, |ui|
            {
                ui.horizontal_wrapped(|ui|
                {
                    for asset_meta in meta.values()
                    {
                        if asset_meta.parent == Some( self.current_directory )
                        {
                            self.show_asset2(ui, asset_meta, &mut hovered_asset, actions);
                        }
                    }
                });
            });

            if self.hovered_asset.is_some() && hovered_asset == false
            {
                actions.push( ContentBrowserViewportAction::NoLongerHoveringAssets );
            }
        });

        if !ui.max_rect().contains(user_inputs.mouse_position)
        {
            return;
        }
    }

    fn show_asset2(&self, ui: &mut egui::Ui, asset_meta: &AssetMeta, hovered_asset: &mut bool, actions: &mut Vec<ContentBrowserViewportAction>)
    {
        ui.vertical(|ui|
        {
            ui.allocate_ui_with_layout(
                                        egui::Vec2::new(70.0, 0.0), 
                                        egui::Layout::top_down(egui::Align::Center),
                                        |ui|
            {
                let asset_icon = match asset_meta.kind
                {
                    AssetKind::Folder => String::from("📁"),
                    _ => String::from("📃"),
                };

                let asset_is_selected_or_hovered = Some( asset_meta.id ) == self.selected_asset || Some( asset_meta.id ) == self.hovered_asset;

                let selectable_label = egui::Button::selectable(asset_is_selected_or_hovered, egui::RichText::new(asset_icon.clone()).font(egui::FontId::proportional(70.0))).sense(egui::Sense::click_and_drag());
                let selectable_asset_response = ui.add(selectable_label).on_hover_text(asset_meta.name.clone());

                let potentially_shortened_asset_name = shorten_text(asset_meta.name.clone(), 11);
                ui.label(potentially_shortened_asset_name);

                if selectable_asset_response.hovered()
                {
                    actions.push( ContentBrowserViewportAction::HoveringAsset( asset_meta.id ));
                    *hovered_asset = true;
                }

                if selectable_asset_response.clicked()
                {
                    actions.push( ContentBrowserViewportAction::SelectedAsset( asset_meta.id ) );
                }

                if selectable_asset_response.double_clicked()
                {
                    match asset_meta.kind
                    {
                        AssetKind::Folder =>
                        {
                            actions.push( ContentBrowserViewportAction::ChangeDirectory( asset_meta.id ) );
                        },
                        _ => {},
                    }
                }
            });
        });
    }

    fn show_asset(&mut self, ui: &mut egui::Ui, asset_path: &PathBuf, studio_context: &mut StudioContext, project_path: &PathBuf)
    {
        let mut is_asset_selected = false;
        if self.selected_asset.is_some()
        {
            // is_asset_selected = *self.selected_asset.as_ref().unwrap() == *asset_path;
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

                let asset_relative_path = asset_path.strip_prefix(project_path).unwrap().to_path_buf();

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
                    // self.selected_asset = Some( asset_relative_path.clone() );
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

                        // self.current_directory = asset_relative_path;
                        // self.selected_asset = None;
                        return;
                    }

                    // let asset_id = studio_context.get_project().assets.path_to_asset_id.get(&asset_relative_path);

                    // if asset_id.is_none()
                    // {
                    //     panic!("Content browser tried to access an id which is not in the asset"); // @TODO, find a proper way of handling this
                    // }

                    // let asset_meta = studio_context.get_project().assets.meta.get(asset_id.unwrap()).unwrap();

                    // match asset_meta.kind
                    // {
                    //     AssetKind::NodeGraph => { studio_context.request_new_viewport_at_first_docking_leaf( Viewport::Graph { graph_viewport: GraphViewport::new(asset_meta.id) }) },
                    //     AssetKind::Image => { studio_context.request_new_viewport_at_first_docking_leaf( Viewport::ImageViewer { image_asset_id: asset_meta.id }) },
                    // }

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
            
            // project.assets.rename_file(original_path, new_path);

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

    fn show_quick_feature_window(&mut self, ui: &mut egui::Ui, mouse_position_when_activated: &egui::Pos2)
    {
        // let project = studio_context.get_project_mut();

        // let project_directory = &project.location;
        // let full_directory_path = project_directory.join(&self.current_directory);
        // let assets = &mut project.assets;
        
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
                // let new_asset_path = full_directory_path.join("unamed");
                // assets.create_folder(&new_asset_path);

                // self.renaming_file = Some( ViewportRenameState::new( &new_asset_path ) );
                self.quick_menu = None;
            };

            if ui.add(egui::Button::new("create graph").min_size(egui::Vec2 {x: 190.0, y: 20.0})).clicked() 
            {
                // let asset_id = assets.create_asset(project_directory, &self.current_directory, AssetKind::Graph, None);

                // if asset_id.is_some()
                // {
                //     // self.renaming_file = Some( ViewportRenameState::new( &new_asset_path ) ); // @TODO, add this back, but it needs to use asset id instead of path
                // }

                self.quick_menu = None;
            };

            if self.selected_asset.is_some()
            {
                if ui.add(egui::Button::new("rename file").min_size(egui::Vec2 {x: 190.0, y: 20.0})).clicked() 
                {
                    // @TODO, more dangerous unwraps without checks here
                    // self.renaming_file = Some( ViewportRenameState::new( self.selected_asset.as_ref().unwrap() ) );
                    self.quick_menu = None;
                };
            }
        });
    }

    pub fn process_actions(&mut self, actions: Vec<ContentBrowserViewportAction>)
    {
        for action in actions
        {
            match action
            {
                ContentBrowserViewportAction::ChangeDirectory( new_directory_asset_id ) =>
                {
                    self.current_directory = new_directory_asset_id;
                },
                ContentBrowserViewportAction::HoveringAsset( hovered_asset_id ) =>
                {
                    self.hovered_asset = Some( hovered_asset_id );
                },
                ContentBrowserViewportAction::NoLongerHoveringAssets =>
                {
                    self.hovered_asset = None;
                },
                ContentBrowserViewportAction::SelectedAsset( selected_asset_id ) =>
                {
                    self.selected_asset = Some(selected_asset_id);
                },
            }
        }
        
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

#[derive(Clone, Serialize, Deserialize, Debug)]
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

fn show_content_browser_debug_info(content_browser_viewport: &mut ContentBrowserViewport, ui: &mut egui::Ui)
{
    let viewport_rect = ui.max_rect();

    let debug_layer = egui::LayerId::new(
        egui::Order::Foreground,
        egui::Id::new("content_browser_debug_info"), // @TODO, this id needs to be unique to viewport
    );

    let painter = ui.layer_painter(debug_layer)
                    .with_clip_rect(viewport_rect);

    let debug_info = format!(
        "\
        Content Browser Viewport
        current_directory: {:?}
        selected_asset: {:?}
        quick_menu: {:?}
        renaming_file: {:?}"
        , content_browser_viewport.current_directory
        , content_browser_viewport.selected_asset
        , content_browser_viewport.quick_menu
        , content_browser_viewport.renaming_file
    );

    painter.text(
        viewport_rect.left_top() + egui::vec2(10.0, 10.0),
        egui::Align2::LEFT_TOP,
        debug_info,
        egui::FontId::monospace(14.0),
        egui::Color32::RED,
    );
}

#[derive(Clone)]
pub enum ContentBrowserViewportAction
{
    ChangeDirectory( AssetId ),
    HoveringAsset( AssetId ),
    NoLongerHoveringAssets,
    SelectedAsset ( AssetId ),
}
