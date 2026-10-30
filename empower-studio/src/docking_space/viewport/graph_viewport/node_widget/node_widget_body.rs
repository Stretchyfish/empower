use std::collections::VecDeque;

use empower_engine::node_graph::{Node, NodeGraphKey, node::{NodeKind, SettingsKind}};

use crate::docking_space::viewport::graph_viewport::{GraphViewportAction, area_select::AreaSelect};

// mod editable_node_state;

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
            graph_viewport_title: &String, 
            graph_viewport_actions: &mut VecDeque<GraphViewportAction>,
            area_select: &mut Option<AreaSelect>,
            node_size: &egui::Vec2,
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

    let settings_rect = egui::Rect::from_min_max(egui::pos2(node_rect.min.x, title_box_rect.max.y), node_rect.max);
    let settings_height = show_settings(ui, node_key, &mut node.kind, graph_viewport_title, graph_viewport_actions, settings_rect);

    title_box_rect.size().y + settings_height
}

fn show_settings(
    ui: &mut egui::Ui,
    node_key: &NodeGraphKey,
    node_kind: &mut NodeKind,
    graph_viewport_title: &String,
    graph_viewport_actions: &mut VecDeque<GraphViewportAction>,
    settings_rect: egui::Rect,
) -> f32
{
    let mut settings_height = NODE_EDIT_GAP; // @TODO, update this in the future to depend on the Setting
    for (setting_index, setting) in node_kind.settings().into_iter().enumerate()
    {
        let row_position = egui::pos2(settings_rect.min.x + NODE_EDIT_AND_LABEL_BUFFER, settings_rect.min.y + settings_height);
        let changed = show_setting(ui, setting, row_position, settings_rect.width(), graph_viewport_title, node_key, setting_index);

        if changed
        {
            graph_viewport_actions.push_back(GraphViewportAction::NodeSettingsChanged { node_key: *node_key });
        }

        settings_height += 50.0;
    }

    settings_height
}

fn show_setting(
    ui: &mut egui::Ui,
    setting: SettingsKind<'_>,
    row_position: egui::Pos2,
    node_width: f32,
    graph_viewport_title: &String,
    node_key: &NodeGraphKey,
    setting_index: usize,
) -> bool
{
    match setting
    {
        SettingsKind::Text { label, value } =>
        {
            let mut row_ui = setting_row(ui, row_position, node_width - NODE_EDIT_AND_LABEL_BUFFER * 2.0, egui::Align::Center);
            row_ui.label(egui::RichText::new(label).size(30.0));
            let width = row_ui.available_width();
            row_ui.add_sized([width, 40.0], egui::TextEdit::singleline(value).font(egui::FontId::proportional(30.0))).changed()
        },
        SettingsKind::Slider { label, value, min, max } =>
        {
            let mut row_ui = setting_row(ui, row_position, node_width - NODE_EDIT_AND_LABEL_BUFFER * 2.0, egui::Align::Center);
            row_ui.label(egui::RichText::new(label).size(30.0));
            row_ui.add(egui::Slider::new(value, min..=max)).changed()
        },
        SettingsKind::Choice { label, value, options } =>
        {
            let row_width = (ui.clip_rect().right() - row_position.x).max(0.0);
            let mut row_ui = setting_row(ui, row_position, row_width, egui::Align::Min);
            let style = row_ui.style_mut();
            style.text_styles.insert(egui::TextStyle::Button, egui::FontId::proportional(35.0));
            style.visuals.override_text_color = Some(egui::Color32::WHITE);
            style.spacing.button_padding.y = 0.0;

            row_ui.label(egui::RichText::new(label).font(egui::FontId::proportional(35.0)).color(egui::Color32::WHITE));
            row_ui.add_space(NODE_EDIT_GAP * 2.0);
            let previous_value = value.clone();
            egui::ComboBox::from_id_salt((graph_viewport_title, node_key, setting_index))
                .selected_text(value.clone())
                .show_ui(&mut row_ui, |ui|
                {
                    for option in options
                    {
                        ui.selectable_value(value, option.to_string(), option);
                    }
                });
            *value != previous_value
        },
    }
}

fn setting_row(ui: &mut egui::Ui, position: egui::Pos2, width: f32, alignment: egui::Align) -> egui::Ui
{
    let rect = egui::Rect::from_min_size(position, egui::vec2(width, 50.0));
    ui.new_child(
        egui::UiBuilder::new()
            .max_rect(rect)
            .layout(egui::Layout::left_to_right(alignment)),
    )
}
