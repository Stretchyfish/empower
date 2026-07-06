use std::collections::{BTreeMap, HashMap, VecDeque};

use empower_engine::{assets::{AssetId, AssetKind}, node_graph::{Node, NodeEdit, NodeGraphKey}};

use crate::docking_space::viewport::graph_viewport::{GraphViewportAction, area_select::AreaSelect};

const NODE_BODY_COLOR: egui::Color32 = egui::Color32::from_rgb(63, 63, 63);

const NODE_BOTTOM_RECT_HEIGHT: f32 = 20.0;

const NODE_BODY_TITLE_AREA_OVERLAP: f32 = 12.0; // Due to the drawing of the body in 3 steps, a bit of overlap is done to smoothen
const NODE_BODY_BUTTON_AREA_OVERLAP: f32 = 12.0;

const NODE_EDIT_GAP: f32 = 10.0;
const NODE_EDIT_AND_LABEL_BUFFER: f32 = 20.0;

pub fn show(
            ui: &mut egui::Ui, 
            node_key: &NodeGraphKey,
            node: &mut Node,
            viewport_graph_id: &AssetId,
            graph_viewport_title: &String, 
            graph_viewport_actions: &mut VecDeque<GraphViewportAction>,
            area_select: &mut Option<AreaSelect>,
            node_size: &egui::Vec2,
            node_graph_names: &HashMap<AssetId, String>,
            image_names: &HashMap<AssetId, String>,
            developer_mode: &bool,
        ) -> f32
{
    let node_rect = egui::Rect::from_min_size(node.position, *node_size);

    if area_select.is_some()
    {
        area_select.as_mut().unwrap().check_if_node_is_inside_area_select_and_add_if_it_is(&node_key, &node_rect);
    }

    let title_text_font_size = 40.0;

    let mut node_title = node.kind.name().to_string();

    if *developer_mode
    {
       node_title = format!("{} [{}]", node_title, node_key.to_string());
    }

    let text_size = ui.painter()
                        .layout_no_wrap(
                            node_title.to_string(),
                            egui::FontId::proportional(title_text_font_size),
                            egui::Color32::YELLOW,
                        )
                        .size();
    
        
    let title_box_rect = egui::Rect::from_min_size(
            node_rect.min,
            egui::Vec2 {
                x: node_rect.size().x,
                y: text_size.y * 2.0,
            },
    );
    
    let node_title_pos = egui::Pos2 {
        x: title_box_rect.center().x,
        y: title_box_rect.min.y
            + (title_box_rect.size().y - NODE_BODY_TITLE_AREA_OVERLAP) / 2.0,
    };
    
    let mut title_rect_color = egui::Color32::from_rgb(50, 50, 50);

    let node_title_reponse = ui.interact(
        title_box_rect,
        egui::Id::new(graph_viewport_title.to_owned() + "_node_body_" + node_key.to_string().as_str()),
        egui::Sense::click_and_drag(),
    );

    if node_title_reponse.hovered() // Important that this is done before clicked
    {
        title_rect_color = egui::Color32::from_rgb(40, 40, 40);
    }

    if node_title_reponse.clicked()
    {
        graph_viewport_actions.push_back( GraphViewportAction::ClickedNodeTitle { node_key: *node_key });
    }

    ui.painter().rect(
        title_box_rect,
        6.0,
        title_rect_color,
        egui::Stroke::NONE,
        egui::StrokeKind::Inside,
    );

    ui.painter().text(
        node_title_pos,
        egui::Align2::CENTER_CENTER,
        node_title,
        egui::FontId::proportional(title_text_font_size),
        egui::Color32::WHITE,
    );

    let node_rect_round_bottom = egui::Rect::from_min_size(
        egui::Pos2 {
            x: node_rect.min.x,
            y: node_rect.max.y - NODE_BOTTOM_RECT_HEIGHT,
            // y: node_rect.max.y,
        },
        egui::Vec2 {
            x: node_rect.size().x,
            y: NODE_BOTTOM_RECT_HEIGHT,
        },
    );

    ui.painter().rect(
        node_rect_round_bottom,
        6.0,
        NODE_BODY_COLOR,
        egui::Stroke::NONE,
        egui::StrokeKind::Inside,
    );
    
    let node_rect_without_title_and_bottom = egui::Rect::from_min_size(
        egui::Pos2 {
            x: node_rect.min.x,
            y: node_rect.min.y + title_box_rect.size().y - NODE_BODY_TITLE_AREA_OVERLAP,
        },
        egui::Vec2 {
            x: node_rect.size().x,
            y: node_rect.size().y
                                    - title_box_rect.size().y + NODE_BODY_TITLE_AREA_OVERLAP
                                    - node_rect_round_bottom.size().y + NODE_BODY_BUTTON_AREA_OVERLAP,
        },
    );

    ui.painter().rect(
        node_rect_without_title_and_bottom,
        0.0,
        NODE_BODY_COLOR,
        egui::Stroke::NONE,
        egui::StrokeKind::Inside,
    );

    let node_edits = node.kind.node_edits();

    if node_edits.is_none()
    {
        return title_box_rect.size().y;
    }

    let node_edits = node_edits.unwrap();

    let mut vertical_offset = title_box_rect.size().y + NODE_EDIT_GAP;

    for (index, edit) in node_edits.iter_mut().enumerate()
    {
        let edit_position = egui::pos2(title_box_rect.min.x, title_box_rect.min.y + vertical_offset);
        
        let (changed, height) = match edit
        {
            NodeEdit::Text { label, text, parseble } => draw_text_node_edit(ui, &edit_position, label, text, *parseble),
            NodeEdit::CheckBox { toggle: _ } => todo!(),
            NodeEdit::GraphViewportOpener { graph_id } => draw_graph_viewport_opener(ui, &edit_position, graph_id, graph_viewport_actions),
            NodeEdit::AssetSelector { asset_id, kind } => draw_asset_selector_edit(ui, &edit_position, asset_id, viewport_graph_id, kind, node_graph_names, image_names),
        };

        if changed
        {
            graph_viewport_actions.push_back( GraphViewportAction::NodeEditWasChanged { node_key: *node_key, node_edit_index: index });
        }

        vertical_offset += height + NODE_EDIT_GAP;
    }

    vertical_offset
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
        AssetKind::Graph => "graph",
        AssetKind::Image => "image",
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
            AssetKind::Graph => node_graph_names.get(&selected_asset.unwrap()).unwrap().clone(), // @TODO, take a second look at this, might be dangerous,
            AssetKind::Image => image_names.get(&selected_asset.unwrap()).unwrap().clone(),
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
        AssetKind::Graph => node_graph_names.into_iter().collect(),
        AssetKind::Image => image_names.into_iter().collect(),
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

