use std::collections::VecDeque;

use crate::{docking_space::{Viewport, viewport::GraphViewport}, studio_context::{Log, StudioContext}, user_inputs::UserInputs};
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
        content_browser_viewport.process_user_inputs(user_inputs, studio_context);
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

    content_browser_viewport.process_actions(actions, studio_context);
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
                studio_context.add_log( Log::info( "not implemented yet"));
            }

            if ui.button("👉").clicked()
            {
                studio_context.add_log( Log::info( "not implemented yet"));
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
        egui::panel::CentralPanel::default().show_inside(ui, |ui|
        {
            if self.quick_menu.is_some()
            {
                self.show_quick_feature_window(ui, &self.quick_menu.unwrap(), studio_context, actions);
            }

            let meta = &studio_context.get_project().assets.meta;
            
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
                            self.show_asset(ui, asset_meta, &mut hovered_asset, actions);
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

    fn show_asset(&mut self, ui: &mut egui::Ui, asset_meta: &AssetMeta, hovered_asset: &mut bool, actions: &mut Vec<ContentBrowserViewportAction>)
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
                    AssetKind::Image => String::from("📷"),
                    _ => String::from("📃"),
                };

                let asset_is_selected_or_hovered = Some( asset_meta.id ) == self.selected_asset || Some( asset_meta.id ) == self.hovered_asset;
                let asset_is_being_renamed = self.renaming_file.is_some() && self.renaming_file.as_ref().unwrap().id == asset_meta.id;

                let selectable_label = egui::Button::selectable(asset_is_selected_or_hovered, egui::RichText::new(asset_icon.clone()).font(egui::FontId::proportional(70.0))).sense(egui::Sense::click_and_drag());
                let selectable_asset_response = ui.add(selectable_label).on_hover_text(asset_meta.name.clone());

                if asset_is_being_renamed
                {
                    let renaming_temporary_name = &mut self.renaming_file.as_mut().unwrap().potential_new_name;

                    ui.text_edit_singleline(renaming_temporary_name)
                    .request_focus();

                    return;
                }

                let potentially_shortened_asset_name = shorten_text(asset_meta.name.clone(), 11);
                ui.label(potentially_shortened_asset_name);

                if selectable_asset_response.drag_started()
                {
                    actions.push( ContentBrowserViewportAction::StartedDraggingAsset( asset_meta.id ));
                }

                if selectable_asset_response.drag_stopped()
                {
                    actions.push( ContentBrowserViewportAction::StoppedDraggingAsset );
                }

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
                            actions.push( ContentBrowserViewportAction::ClearSelectedAsset );
                            actions.push( ContentBrowserViewportAction::ChangeDirectory( asset_meta.id ) );
                        },
                        AssetKind::NodeGraph =>
                        {
                            actions.push( ContentBrowserViewportAction::RequestStudioToOpenViewport( Viewport::Graph { graph_viewport: GraphViewport::new(asset_meta.id) }) );
                        },
                        AssetKind::Image =>
                        {
                            actions.push( ContentBrowserViewportAction::RequestStudioToOpenViewport( Viewport::ImageViewer { image_asset_id: asset_meta.id }) );
                        }
                        _ => {},
                    }
                }
            });
        });
    }

    pub fn process_user_inputs(&mut self, user_inputs: &UserInputs, studio_context: &mut StudioContext)
    {
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

            let successfully_renamed_asset = studio_context.get_project_mut().assets.rename_asset(&rename_file_state.id, self.renaming_file.as_ref().unwrap().potential_new_name.as_str());

            if !successfully_renamed_asset 
            {
                studio_context.add_log( Log::warning("was unable to rename asset"));
                return;
            }

            self.renaming_file = None;
            return;
        }

        // @TODO, update this to be, detect background inside of the viewport instead

        if ( user_inputs.clicked_esp || user_inputs.clicked_primary_mouse_button || user_inputs.clicked_secondary_mouse_button) && self.renaming_file.is_some()
        {
            self.renaming_file = None;
            return;
        }

        // @TODO, add back that left click can stop quick menu

        if self.quick_menu.is_none() && ( user_inputs.clicked_esp || user_inputs.clicked_primary_mouse_button || user_inputs.clicked_secondary_mouse_button) && self.selected_asset.is_some()
        {
            self.selected_asset = None;
            return;
        }
    }

    fn show_quick_feature_window(&mut self, ui: &mut egui::Ui, mouse_position_when_activated: &egui::Pos2, studio_context: &mut StudioContext, actions: &mut Vec<ContentBrowserViewportAction>)
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
            if ui.add(egui::Button::new("create folder").min_size(egui::Vec2 {x: 190.0, y: 20.0})).clicked() 
            {
                let created_folder_result = studio_context.get_project_mut().assets.create_asset( Some( self.current_directory ), AssetKind::Folder, "folder");

                if created_folder_result.is_err()
                {
                    studio_context.add_log( Log::info(created_folder_result.err().unwrap().as_str()) );
                    return;
                }

                self.renaming_file = Some( ViewportRenameState::new(String::new(), created_folder_result.unwrap()));
                self.quick_menu = None;
            };

            if ui.add(egui::Button::new("create graph").min_size(egui::Vec2 {x: 190.0, y: 20.0})).clicked() 
            {
                let created_node_graph_result = studio_context.get_project_mut().assets.create_asset( Some( self.current_directory ), AssetKind::NodeGraph, "unamed.json");

                if created_node_graph_result.is_err()
                {
                    studio_context.add_log( Log::info(created_node_graph_result.err().unwrap().as_str()) );
                    return;
                }

                let asset_meta = studio_context.get_project().assets.meta.get(created_node_graph_result.as_ref().unwrap()).unwrap();

                self.renaming_file = Some( ViewportRenameState::new(asset_meta.name.clone(), created_node_graph_result.unwrap()));
                self.quick_menu = None;
            };

            if self.selected_asset.is_some()
            {
                if ui.add(egui::Button::new("rename file").min_size(egui::Vec2 {x: 190.0, y: 20.0})).clicked() 
                {
                    let asset_meta = studio_context.get_project().assets.meta.get( self.selected_asset.as_ref().unwrap() );

                    if asset_meta.is_none()
                    {
                        studio_context.add_log( Log::warning("cannot get asset_meta and rename file"));
                        return;
                    }

                    actions.push( ContentBrowserViewportAction::RenamingFile( ViewportRenameState::new( asset_meta.unwrap().name.clone(), self.selected_asset.unwrap() ) ));
                    
                    // self.renaming_file = Some( ViewportRenameState::new( asset_meta.unwrap().name.clone(), self.selected_asset.unwrap() ));
                    self.quick_menu = None;
                };
            }
        });
    }

    pub fn process_actions(&mut self, actions: Vec<ContentBrowserViewportAction>, studio_context: &mut StudioContext)
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
                ContentBrowserViewportAction::ClearSelectedAsset =>
                {
                    self.selected_asset = None;
                },
                ContentBrowserViewportAction::RenamingFile(viewport_rename_state) =>
                {
                    self.renaming_file = Some( viewport_rename_state );
                    self.quick_menu = None;
                },
                ContentBrowserViewportAction::RequestStudioToOpenViewport(viewport) =>
                {
                    studio_context.request_new_viewport_at_first_docking_leaf( viewport );
                },
                ContentBrowserViewportAction::StartedDraggingAsset(asset_id) =>
                {
                    studio_context.request_begin_dragging_asset(asset_id);
                },
                ContentBrowserViewportAction::StoppedDraggingAsset =>
                {
                    studio_context.request_stop_dragging_asset();
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
pub struct ViewportRenameState
{
    potential_new_name: String,
    id: AssetId,
}

impl ViewportRenameState
{
    pub fn new(name: String, asset_id: AssetId) -> Self
    {
        Self
        {
            potential_new_name: name,
            id: asset_id, 
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
    StartedDraggingAsset( AssetId ),
    StoppedDraggingAsset,
    NoLongerHoveringAssets,
    SelectedAsset ( AssetId ),
    ClearSelectedAsset,
    RenamingFile( ViewportRenameState ),
    RequestStudioToOpenViewport ( Viewport ),
}
