use std::collections::{BTreeMap, HashMap, VecDeque};

use empower_engine::{assets::{AssetId, AssetKind}, node_graph::node::{NodeKind, node_kind::LoopMode}, value::Value};

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
            _ => EditableNodeState::None,
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, edit_position: egui::Pos2, viewport_graph_id: &AssetId, node_graph_names: &HashMap<AssetId, String>, image_names: &HashMap<AssetId, String>, graph_viewport_actions: &mut VecDeque<GraphViewportAction>) -> ShowEditableNodeStateResult 
    {
        match self
        {
            EditableNodeState::None => ShowEditableNodeStateResult { changed: false, size: egui::Vec2::ZERO },
            EditableNodeState::Loop( mode ) =>
            {
                let (changed1, height1) = draw_loop_mode_type_selector(ui, &edit_position, mode);
                ShowEditableNodeStateResult { changed: changed1, size: egui::vec2(0.0, NODE_EDIT_GAP + height1 ) }
            },
            EditableNodeState::List( text1, text2 ) =>
            {
                let (changed1, height1) = draw_text_node_edit(ui, &edit_position, &"test:".to_string(), text1, true);
                let (changed2, height2) = draw_value_type_selector(ui, &(edit_position + egui::vec2( 0.0, height1 + NODE_EDIT_GAP )), text2);
                ShowEditableNodeStateResult { changed: (changed1 || changed2), size: egui::vec2(0.0, NODE_EDIT_GAP + height1 + height2 ) }
            },
            EditableNodeState::Image( asset_id ) =>
            {
                let (changed, _) = draw_asset_selector_edit(ui, &edit_position, asset_id, viewport_graph_id, &AssetKind::Image, node_graph_names, image_names);
                ShowEditableNodeStateResult { changed, size: egui::vec2(0.0, NODE_EDIT_GAP ) }
            },
            EditableNodeState::SubGraph( asset_id ) =>
            {
                let (changed, height1) = draw_asset_selector_edit(ui, &edit_position, asset_id, viewport_graph_id, &AssetKind::NodeGraph, node_graph_names, image_names);
                let (_, height2) = draw_graph_viewport_opener(ui, &(edit_position + egui::vec2(0.0, height1 + NODE_EDIT_GAP)), asset_id, graph_viewport_actions);

                ShowEditableNodeStateResult { changed, size: egui::vec2(0.0, NODE_EDIT_GAP + height1 + height2 ) }
            },
        }
    }

    pub fn sync_with_node_state(&self, node_kind: &mut NodeKind) -> NodeSyncResponse
    {
        match self
        {
            EditableNodeState::None => NodeSyncResponse::Nothing,
            EditableNodeState::List( number_of_input_ports_text, value_type_text ) =>
            {
                let list_state = match node_kind
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
                let image_state = match node_kind
                {
                    NodeKind::Image( image_state ) => image_state,
                    _ => panic!("tries to parse incompatible state from editable state"),
                };

                image_state.image_asset_id = *image_asset_id;

                NodeSyncResponse::NodesStructureChanged
            },
            EditableNodeState::SubGraph( graph_asset_id ) =>
            {
                let sub_graph_state = match node_kind
                {
                    NodeKind::SubGraph( sub_graph_state ) => sub_graph_state,
                    _ => panic!("tries to parse incompatible state from editable state"),
                };

                sub_graph_state.graph_asset_id = *graph_asset_id;

                NodeSyncResponse::NodesStructureChanged
            },
            EditableNodeState::Loop(mode) =>
            {
                let loop_mode = match node_kind
                {
                    NodeKind::Loop( mode ) => mode,
                    _ => panic!("tries to parse incompatible state from editable state"),
                };

                *loop_mode = mode.clone(); // @TODO, look into if this clone can be removed

                NodeSyncResponse::NodesStructureChanged
            },
        }
    }
}

fn draw_loop_mode_type_selector(ui: &mut egui::Ui, edit_position: &egui::Pos2, current_mode: &mut LoopMode) -> (bool, f32)
{
    let label_position = *edit_position + egui::Vec2 { x: NODE_EDIT_AND_LABEL_BUFFER, y: 0.0 };

    let painted_text = ui.painter().text(
        label_position,
        egui::Align2::LEFT_TOP,
        "value: ",
        egui::FontId::proportional(35.0),
        egui::Color32::WHITE,
    );
    
    let mode_before = current_mode.clone();

    let combo_rect = egui::Rect::from_min_size(
        egui::pos2( label_position.x + painted_text.size().x + NODE_EDIT_GAP * 2.0, label_position.y + painted_text.size().y / 2.0),
        egui::Vec2::INFINITY
    );

    let mut child_ui = ui.new_child(egui::UiBuilder::new().max_rect(combo_rect));
    egui::ComboBox::from_id_salt("enum box selector") // @TODO, make ids unique, otherwise it will have conflicts later
    .selected_text( mode_before.to_string() ) // @TODO, figure out if this is needed
    .show_ui(&mut child_ui, |ui|
    {
        ui.selectable_value( current_mode, LoopMode::Forever, String::from("forever"));
        ui.selectable_value( current_mode, LoopMode::Range, String::from("ranged"));
    });
    
    (mode_before != *current_mode, 40.0)
}

fn draw_value_type_selector(ui: &mut egui::Ui, edit_position: &egui::Pos2, current_value_text: &mut String) -> (bool, f32)
{
    let label_position = *edit_position + egui::Vec2 { x: NODE_EDIT_AND_LABEL_BUFFER, y: 0.0 };

    let painted_text = ui.painter().text(
        label_position,
        egui::Align2::LEFT_TOP,
        "value: ",
        egui::FontId::proportional(35.0),
        egui::Color32::WHITE,
    );
    
    let state_before_change = current_value_text.clone();

    let combo_rect = egui::Rect::from_min_size(
        egui::pos2( label_position.x + painted_text.size().x + NODE_EDIT_GAP * 2.0, label_position.y + painted_text.size().y / 2.0),
        egui::Vec2::INFINITY
    );

    let mut child_ui = ui.new_child(egui::UiBuilder::new().max_rect(combo_rect));
    egui::ComboBox::from_id_salt("enum box selector") // @TODO, make ids unique, otherwise it will have conflicts later
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

fn draw_asset_selector_edit(ui: &mut egui::Ui, edit_position: &egui::Pos2, selected_asset: &mut Option<AssetId>, viewport_graph_id: &AssetId, asset_kind: &AssetKind, node_graph_names: &HashMap<AssetId, String>, image_names: &HashMap<AssetId, String>) -> (bool, f32)
{
    let label_position = *edit_position + egui::Vec2 { x: NODE_EDIT_AND_LABEL_BUFFER, y: 0.0 };

    let label = match asset_kind
    {
        AssetKind::NodeGraph => "graph",
        AssetKind::Image => "image",
        AssetKind::Folder => todo!(),
        AssetKind::Json => todo!(),
    };

    let painted_text = ui.painter().text(
        label_position,
        egui::Align2::LEFT_TOP,
        label,
        egui::FontId::proportional(35.0),
        egui::Color32::WHITE,
    );

    let current_asset_name = if selected_asset.is_some()
    {
        match asset_kind
        {
            AssetKind::NodeGraph => node_graph_names.get(&selected_asset.unwrap()).unwrap().clone(), // @TODO, take a second look at this, might be dangerous,
            AssetKind::Image => image_names.get(&selected_asset.unwrap()).unwrap().clone(),
            AssetKind::Folder => todo!(),
            AssetKind::Json => todo!(),
        }
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

    let sorted_names: BTreeMap<&AssetId, &String> = match asset_kind
    {
        AssetKind::NodeGraph => node_graph_names.into_iter().collect(),
        AssetKind::Image => image_names.into_iter().collect(),
        AssetKind::Folder => todo!(),
        AssetKind::Json => todo!(),
    };

    let mut child_ui = ui.new_child(egui::UiBuilder::new().max_rect(combo_rect));
    egui::ComboBox::from_id_salt("asset selector") // @TODO, make ids unique, otherwise it will have conflicts later
    .selected_text( current_asset_name )
    .show_ui(&mut child_ui, |ui|
    {
        for (id, text) in sorted_names
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

