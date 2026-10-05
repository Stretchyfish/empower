use std::{collections::{BTreeMap, HashMap, VecDeque}, path::PathBuf, str::FromStr};

use empower_engine::{assets::{AssetId, AssetKind, AssetMeta, Assets}, node_graph::{NodeGraphKey, node::{NodeKind, node_kind::{LoopMode, ReadFileState, SubGraphState}}}, value::Value};
use rfd::FileDialog;

use crate::docking_space::viewport::graph_viewport::GraphViewportAction;

use super::{NODE_EDIT_GAP, NODE_EDIT_AND_LABEL_BUFFER, NodeSyncResponse};

pub struct ShowEditableNodeStateResult
{
    pub changed: bool,
    pub size: egui::Vec2,
}

#[derive(Clone)]
pub enum EditableNodeState
{
    None,
    Loop( LoopMode ),
    List( String, String ),
    Image( Option<AssetId> ),
    SubGraph( Option<AssetId> ),
    ReadFile( ReadFileState, bool ),
}

impl EditableNodeState
{
    pub fn from(node_kind: &NodeKind) -> Self
    {
        match node_kind
        {
            NodeKind::Loop( mode ) => EditableNodeState::Loop( mode.clone() ),
            NodeKind::List( state ) => EditableNodeState::List( state.size.to_string(), state.value_type.type_string() ),
            NodeKind::Image( state ) => EditableNodeState::Image( state.image_asset_id ),
            NodeKind::SubGraph( state ) => EditableNodeState::SubGraph( state.graph_asset_id ),
            NodeKind::ReadFile( state ) => EditableNodeState::ReadFile( state.clone(), true ),
            _ => EditableNodeState::None,
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, edit_position: egui::Pos2, viewport_graph_id: &AssetId, meta: &HashMap<AssetId, AssetMeta>, graph_viewport_actions: &mut VecDeque<GraphViewportAction>, node_key: &NodeGraphKey, viewport_name: &String) -> ShowEditableNodeStateResult 
    {
        match self
        {
            EditableNodeState::None => ShowEditableNodeStateResult { changed: false, size: egui::Vec2::ZERO },
            EditableNodeState::Loop( mode ) =>
            {
                let (changed1, height1) = draw_loop_mode_type_selector(ui, &edit_position, mode, node_key, viewport_name);
                ShowEditableNodeStateResult { changed: changed1, size: egui::vec2(0.0, NODE_EDIT_GAP + height1 ) }
            },
            EditableNodeState::List( text1, text2 ) =>
            {
                let (changed1, height1) = draw_text_node_edit(ui, &edit_position, &"size:".to_string(), text1, true);
                let (changed2, height2) = draw_value_type_selector(ui, &(edit_position + egui::vec2( 0.0, height1 + NODE_EDIT_GAP )), "value", text2,node_key, viewport_name);
                ShowEditableNodeStateResult { changed: (changed1 || changed2), size: egui::vec2(0.0, NODE_EDIT_GAP + height1 + NODE_EDIT_GAP + height2 ) } // @TODO, this approach to size needs an overhaul
            },
            EditableNodeState::Image( asset_id ) =>
            {
                let (changed, height1) = draw_asset_selector_edit(ui, &edit_position, asset_id, viewport_graph_id, &AssetKind::Image, meta,node_key, viewport_name);
                ShowEditableNodeStateResult { changed, size: egui::vec2(0.0, NODE_EDIT_GAP + height1 ) }
            },
            EditableNodeState::SubGraph( asset_id ) =>
            {
                let (changed, height1) = draw_asset_selector_edit(ui, &edit_position, asset_id, viewport_graph_id, &AssetKind::NodeGraph, meta, node_key, viewport_name);
                let (_, height2) = draw_graph_viewport_opener(ui, &(edit_position + egui::vec2(0.0, height1 + NODE_EDIT_GAP)), asset_id, graph_viewport_actions);

                ShowEditableNodeStateResult { changed, size: egui::vec2(0.0, NODE_EDIT_GAP + height1 + NODE_EDIT_GAP + height2 ) }
            },
            EditableNodeState::ReadFile(state, parsable ) =>
            {
                let mut changed = false;
                let mut total_height_offset = 0.0;

                let (changed1, height1) = draw_read_file_type_selector(ui, &edit_position, state, node_key, viewport_name);

                total_height_offset += height1;
                if changed1 // This is not a good approach, need to find a better way
                {
                    changed = true;
                }

                match state
                {
                    ReadFileState::Asset( asset_id ) =>
                    {
                        *parsable = true;

                        let (changed2, height2) = draw_multi_asset_selector(ui, &(edit_position + egui::vec2(0.0, total_height_offset)), asset_id, viewport_graph_id, &vec![AssetKind::Json, AssetKind::Image, AssetKind::NodeGraph], meta,node_key, viewport_name);

                        total_height_offset += height2;
                        if changed2
                        {
                            changed = true;
                        }
                    },
                    ReadFileState::GlobalPath( path ) =>
                    {
                        let (changed2, height2) = draw_path_selector(ui, &(edit_position + egui::vec2(0.0, total_height_offset)), &String::from("path"), path, parsable, PathSelectorType::Global);

                        total_height_offset += height2;
                        if changed2
                        {
                            changed = true;
                        }
                        
                    },
                    ReadFileState::RelativePath( path ) =>
                    {
                        let (changed2, height2) = draw_path_selector(ui, &(edit_position + egui::vec2(0.0, total_height_offset)), &String::from("path"), path, parsable, PathSelectorType::Relative);

                        total_height_offset += height2;
                        if changed2
                        {
                            changed = true;
                        }
                    },
                }
                ShowEditableNodeStateResult { changed, size: egui::vec2(0.0, NODE_EDIT_GAP + height1 + NODE_EDIT_GAP + total_height_offset ) }
            }
        }
    }

    // pub fn sync_with_node_state(&self, node_kind: &mut NodeKind) -> NodeSyncResponse
    pub fn sync_with_node_state(&self, graph_id: &AssetId, node_key: &NodeGraphKey, assets: &mut Assets) -> NodeSyncResponse
    {
        match self
        {
            EditableNodeState::None => NodeSyncResponse::Nothing,
            EditableNodeState::List( number_of_input_ports_text, value_type_text ) =>
                    {
                        let node = assets.get_node_graph_mut(graph_id).unwrap().nodes.get_mut(node_key).unwrap();
                        let list_state = match &mut node.kind
                        {
                            NodeKind::List( list_state ) => list_state,
                            _ => panic!("tries to parse incompatible state from editable state"),
                        };

                        let parsed = number_of_input_ports_text.parse::<usize>();

                        if parsed.is_err()
                        {
                            return NodeSyncResponse::Nothing;
                        }

                        list_state.size = parsed.unwrap();

                        let value = Value::from_type_string(value_type_text);  

                        if value.is_none()
                        {
                            return NodeSyncResponse::Nothing;
                        }

                        list_state.value_type = value.unwrap();

                        NodeSyncResponse::NodesStructureChanged
                    },
            EditableNodeState::Image( image_asset_id ) =>
                    {
                        let node = assets.get_node_graph_mut(graph_id).unwrap().nodes.get_mut(node_key).unwrap();
                        let image_state = match &mut node.kind
                        {
                            NodeKind::Image( image_state ) => image_state,
                            _ => panic!("tries to parse incompatible state from editable state"),
                        };

                        image_state.image_asset_id = *image_asset_id;

                        NodeSyncResponse::NodesStructureChanged
                    },
            EditableNodeState::SubGraph( graph_asset_id ) =>
                    {
                        if graph_asset_id.is_none()
                        {
                            return NodeSyncResponse::Nothing;
                        }
                
                        let new_state = SubGraphState::from( graph_asset_id.unwrap(), &assets.get_node_graph(&graph_asset_id.unwrap()).unwrap() );

                        let node = assets.get_node_graph_mut(graph_id).unwrap().nodes.get_mut(node_key).unwrap();
                
                        match &mut node.kind
                        {
                            NodeKind::SubGraph( sub_graph_state ) => *sub_graph_state = new_state,
                            _ => panic!("tries to parse incompatible state from editable state"),
                        };

                        NodeSyncResponse::NodesStructureChanged
                    },
            EditableNodeState::Loop(mode) =>
                    {
                        let node = assets.get_node_graph_mut(graph_id).unwrap().nodes.get_mut(node_key).unwrap();
                        // let loop_mode = match &mut node.kind
                        match &mut node.kind
                        {
                            NodeKind::Loop( loop_mode ) => *loop_mode = mode.clone(),
                            _ => panic!("tries to parse incompatible state from editable state"),
                        };

                        NodeSyncResponse::NodesStructureChanged
                    },
            EditableNodeState::ReadFile(read_file_state, _) =>
            {
                let node = assets.get_node_graph_mut(graph_id).unwrap().nodes.get_mut(node_key).unwrap();
                match &mut node.kind
                {
                    NodeKind::ReadFile( state ) => *state= read_file_state.clone(),
                    _ => panic!("tries to parse incompatible state from editable state"),
                };

                NodeSyncResponse::Nothing
            },
        }
    }
}

fn draw_loop_mode_type_selector(ui: &mut egui::Ui, edit_position: &egui::Pos2, current_mode: &mut LoopMode, node_key: &NodeGraphKey, viewport_name: &String) -> (bool, f32)
{
    let label_position = *edit_position + egui::Vec2 { x: NODE_EDIT_AND_LABEL_BUFFER, y: 0.0 };

    let painted_text = ui.painter().text(
        label_position,
        egui::Align2::LEFT_TOP,
        "mode: ",
        egui::FontId::proportional(35.0),
        egui::Color32::WHITE,
    );
    
    let mode_before = current_mode.clone();

    let combo_rect = egui::Rect::from_min_size(
        egui::pos2( label_position.x + painted_text.size().x + NODE_EDIT_GAP * 2.0, label_position.y + painted_text.size().y / 2.0),
        egui::Vec2::INFINITY
    );

    let mut child_ui = ui.new_child(egui::UiBuilder::new().max_rect(combo_rect));
    egui::ComboBox::from_id_salt(format!("enum_box_selector_{}_{}", node_key.to_string(), viewport_name) )
    .selected_text( mode_before.to_string() ) // @TODO, figure out if this is needed
    .show_ui(&mut child_ui, |ui|
    {
        ui.selectable_value( current_mode, LoopMode::Forever, String::from("forever"));
        ui.selectable_value( current_mode, LoopMode::Range, String::from("ranged"));
    });
    
    (mode_before != *current_mode, 40.0)
}

fn draw_value_type_selector(ui: &mut egui::Ui, edit_position: &egui::Pos2, label: &str, current_value_text: &mut String, node_key: &NodeGraphKey, viewport_name: &String) -> (bool, f32)
{
    let label_position = *edit_position + egui::Vec2 { x: NODE_EDIT_AND_LABEL_BUFFER, y: 0.0 };

    let painted_text = ui.painter().text(
        label_position,
        egui::Align2::LEFT_TOP,
        label,
        egui::FontId::proportional(35.0),
        egui::Color32::WHITE,
    );
    
    let state_before_change = current_value_text.clone();

    let combo_rect = egui::Rect::from_min_size(
        egui::pos2( label_position.x + painted_text.size().x + NODE_EDIT_GAP * 2.0, label_position.y + painted_text.size().y / 2.0),
        egui::Vec2::INFINITY
    );

    let mut child_ui = ui.new_child(egui::UiBuilder::new().max_rect(combo_rect));
    egui::ComboBox::from_id_salt(format!("enum_box_selector_{}_{}", node_key.to_string(), viewport_name))
    .selected_text( current_value_text.clone() ) // @TODO, figure out if this is needed
    .show_ui(&mut child_ui, |ui|
    {
        // @TODO, probably is a way of automating this
        ui.selectable_value( current_value_text, String::from("integer"), String::from("integer"));
        ui.selectable_value( current_value_text, String::from("float"), String::from("float"));
        ui.selectable_value( current_value_text, String::from("point2d"), String::from("point2d"));
    });
    
    (state_before_change != *current_value_text, 40.0)
}

fn draw_text_node_edit(ui: &mut egui::Ui, edit_position: &egui::Pos2, label: &String, text: &mut String, parseble: bool) -> (bool, f32)
{
    let label_position = *edit_position + egui::Vec2 { x: NODE_EDIT_AND_LABEL_BUFFER, y: 0.0 };

    let painted_text = ui.painter().text(
        label_position,
        egui::Align2::LEFT_TOP,
        label,
        egui::FontId::proportional(35.0),
        egui::Color32::WHITE,
    );

    let text_box_rect = egui::Rect::from_min_size(
                                egui::pos2( label_position.x + painted_text.size().x + NODE_EDIT_GAP, label_position.y),
                                egui::vec2( 100.0, painted_text.size().y ));

    let text_edit_color = if parseble { egui::Color32::WHITE } else { egui::Color32::RED };

    let text_edit = egui::TextEdit::singleline(text)
    .font(egui::FontId::proportional(35.0))
    .text_color(text_edit_color)
    .background_color(egui::Color32::BLACK);

    let response = ui.put(text_box_rect , text_edit);

    (response.changed(), 40.0 )
}

fn draw_asset_selector_edit(ui: &mut egui::Ui, edit_position: &egui::Pos2, selected_asset: &mut Option<AssetId>, viewport_graph_id: &AssetId, asset_kind: &AssetKind, meta: &HashMap<AssetId, AssetMeta>, node_key: &NodeGraphKey, viewport_name: &String) -> (bool, f32)
{
    let label_position = *edit_position + egui::Vec2 { x: NODE_EDIT_AND_LABEL_BUFFER, y: 0.0 };

    let label = match asset_kind
    {
        AssetKind::NodeGraph => "graph",
        AssetKind::Image => "image",
        AssetKind::Folder => todo!(),
        AssetKind::Json => "json",
    };

    let painted_text = ui.painter().text(
        label_position,
        egui::Align2::LEFT_TOP,
        label,
        egui::FontId::proportional(35.0),
        egui::Color32::WHITE,
    );

    let sorted_alterntive_asset_names: BTreeMap<&AssetId, String> = meta.iter().filter(|(_, m)| m.kind == *asset_kind ).map(|(k, m)| (k, m.name.clone()) ).collect();

    let current_asset_name = if selected_asset.is_some()
    {
        sorted_alterntive_asset_names.get(&selected_asset.unwrap()).unwrap().clone()
    }
    else
    {
        "unknown".to_string()
    };
    
    let id_before_change = *selected_asset;

    let combo_rect = egui::Rect::from_min_size(
        egui::pos2( label_position.x + painted_text.size().x + NODE_EDIT_GAP * 2.0, label_position.y + painted_text.size().y / 2.0),
        // egui::vec2(200.0, painted_text.size().y * 2.0)
        egui::Vec2::INFINITY
    );

    let mut child_ui = ui.new_child(egui::UiBuilder::new().max_rect(combo_rect));
    egui::ComboBox::from_id_salt(format!("asset_selector_{}_{}", node_key.to_string(), viewport_name)) 
    .selected_text( current_asset_name )
    .show_ui(&mut child_ui, |ui|
    {
        for (id, text) in sorted_alterntive_asset_names
        {
            if id == viewport_graph_id
            {
                continue;
            }
            
            ui.selectable_value( selected_asset, Some( *id ), text);
        }
    });
    // egui::Area::new("graph selector".into())
    // .fixed_pos( egui::pos2( label_position.x + painted_text.size().x + NODE_EDIT_GAP, label_position.y) )
    // .show(&mut child_ui.ctx(), |ui|
    // {

    // });
    
    (id_before_change != *selected_asset, 40.0)
}

fn draw_multi_asset_selector(ui: &mut egui::Ui, edit_position: &egui::Pos2, selected_asset: &mut Option<AssetId>, viewport_graph_id: &AssetId, asset_kinds: &Vec<AssetKind>, meta: &HashMap<AssetId, AssetMeta>, node_key: &NodeGraphKey, viewport_name: &String) -> (bool, f32)
{
    let label_position = *edit_position + egui::Vec2 { x: NODE_EDIT_AND_LABEL_BUFFER, y: 0.0 };

    let label = "asset";

    let painted_text = ui.painter().text(
        label_position,
        egui::Align2::LEFT_TOP,
        label,
        egui::FontId::proportional(35.0),
        egui::Color32::WHITE,
    );

    let sorted_alterntive_asset_names: BTreeMap<&AssetId, String> = meta.iter().filter(|(_, m)| asset_kinds.iter().any(|asset_kind| *asset_kind == m.kind ) ).map(|(k, m)| (k, m.name.clone()) ).collect();

    let current_asset_name = if selected_asset.is_some()
    {
        sorted_alterntive_asset_names.get(&selected_asset.unwrap()).unwrap().clone()
    }
    else
    {
        "unknown".to_string()
    };
    
    let id_before_change = *selected_asset;

    let combo_rect = egui::Rect::from_min_size(
        egui::pos2( label_position.x + painted_text.size().x + NODE_EDIT_GAP * 2.0, label_position.y + painted_text.size().y / 2.0),
        // egui::vec2(200.0, painted_text.size().y * 2.0)
        egui::Vec2::INFINITY
    );

    let mut child_ui = ui.new_child(egui::UiBuilder::new().max_rect(combo_rect));
    egui::ComboBox::from_id_salt(format!("asset_selector_{}_{}", node_key.to_string(), viewport_name)) 
    .selected_text( current_asset_name )
    .show_ui(&mut child_ui, |ui|
    {
        for (id, text) in sorted_alterntive_asset_names
        {
            if id == viewport_graph_id
            {
                continue;
            }
            
            ui.selectable_value( selected_asset, Some( *id ), text);
        }
    });
    // egui::Area::new("graph selector".into())
    // .fixed_pos( egui::pos2( label_position.x + painted_text.size().x + NODE_EDIT_GAP, label_position.y) )
    // .show(&mut child_ui.ctx(), |ui|
    // {

    // });
    
    (id_before_change != *selected_asset, 40.0)
}

fn draw_graph_viewport_opener(ui: &mut egui::Ui, edit_position: &egui::Pos2, graph_id: &Option<AssetId>, graph_viewport_actions: &mut VecDeque<GraphViewportAction>) -> (bool, f32)
{
    if graph_id.is_none()
    {
        return (false, 40.0);
    }

    let button_position = *edit_position + egui::Vec2 { x: NODE_EDIT_AND_LABEL_BUFFER, y: 0.0 };

    let button_rect = egui::Rect::from_min_size(
        button_position,
        egui::vec2(200.0, 40.0)
    );

    let button = egui::Button::new("open viewport");

    let response = ui.put(button_rect, button);

    if response.clicked()
    {
        graph_viewport_actions.push_back( GraphViewportAction::RequestNewGraphViewportOrFocus { graph_id: graph_id.unwrap() });
    }
    
    (false, 40.0)
}

fn draw_read_file_type_selector(ui: &mut egui::Ui, edit_position: &egui::Pos2, current_mode: &mut ReadFileState, node_key: &NodeGraphKey, viewport_name: &String) -> (bool, f32)
{
    let label_position = *edit_position + egui::Vec2 { x: NODE_EDIT_AND_LABEL_BUFFER, y: 0.0 };

    let painted_text = ui.painter().text(
        label_position,
        egui::Align2::LEFT_TOP,
        "location: ",
        egui::FontId::proportional(35.0),
        egui::Color32::WHITE,
    );
    
    let mode_before = current_mode.clone();

    let combo_rect = egui::Rect::from_min_size(
        egui::pos2( label_position.x + painted_text.size().x + NODE_EDIT_GAP * 2.0, label_position.y + painted_text.size().y / 2.0),
        egui::Vec2::INFINITY
    );

    let mut child_ui = ui.new_child(egui::UiBuilder::new().max_rect(combo_rect));
    egui::ComboBox::from_id_salt(format!("read_file_selector_{}_{}", node_key.to_string(), viewport_name) )
    .selected_text( mode_before.to_string() ) // @TODO, figure out if this is needed
    .show_ui(&mut child_ui, |ui|
    {
        ui.selectable_value( current_mode, ReadFileState::Asset( None ), String::from("asset"));
        ui.selectable_value( current_mode, ReadFileState::GlobalPath( String::new() ), String::from("global_path"));
        ui.selectable_value( current_mode, ReadFileState::RelativePath( String::new() ), String::from("relative"));
    });
    
    (mode_before != *current_mode, 40.0)
}

enum PathSelectorType
{
    Global,
    Relative
}

fn draw_path_selector(ui: &mut egui::Ui, edit_position: &egui::Pos2, label: &String, text: &mut String, parseble: &mut bool, path_selector_type: PathSelectorType) -> (bool, f32)
{
    let label_position = *edit_position + egui::Vec2 { x: NODE_EDIT_AND_LABEL_BUFFER, y: 0.0 };

    let painted_text = ui.painter().text(
        label_position,
        egui::Align2::LEFT_TOP,
        label,
        egui::FontId::proportional(35.0),
        egui::Color32::WHITE,
    );

    let text_box_rect = egui::Rect::from_min_size(
                                egui::pos2( label_position.x + painted_text.size().x + NODE_EDIT_GAP, label_position.y),
                                egui::vec2( 100.0, painted_text.size().y ));

    let text_edit_color = if *parseble { egui::Color32::WHITE } else { egui::Color32::RED };

    let text_edit = egui::TextEdit::singleline(text)
    .font(egui::FontId::proportional(20.0))
    .text_color(text_edit_color)
    .background_color(egui::Color32::BLACK);

    let response = ui.put(text_box_rect , text_edit);

    let path_parsed = PathBuf::from_str(text);

    *parseble = false;
    if path_parsed.is_ok()
    {
        if path_parsed.unwrap().exists()
        {
            *parseble = true;
        }
    }

    let button_position = *edit_position + egui::Vec2 { x: NODE_EDIT_AND_LABEL_BUFFER * 2.0 + 170.0, y: 0.0 }; // Figure out a better way to handle this offset

    let button_rect = egui::Rect::from_min_size(
        button_position,
        egui::vec2(80.0, 40.0)
    );

    let button = egui::Button::new("select");

    let response2 = ui.put(button_rect, button);

    if response2.clicked()
    {
        match path_selector_type
        {
            PathSelectorType::Global =>
            {
                let file_path = FileDialog::new()
                    .set_title("select file")
                    .pick_file();
        
                if file_path.is_some()
                {
                    *text = file_path.unwrap().to_string_lossy().to_string();
                }
            },
            PathSelectorType::Relative =>
            {
                let file_path = FileDialog::new()
                    .set_title("select file")
                    .pick_file();
        
                if file_path.is_some()
                {
                    *text = file_path.unwrap().to_string_lossy().to_string(); // @TODO, make it actually work with relative paths
                }
            },
        }
    }

    (response.changed(), 40.0 )
}

