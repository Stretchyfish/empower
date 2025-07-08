use empower_node_graph::port::input_port;
use empower_node_graph::EmpowerData;
use empower_node_graph::EmpowerKey;
use empower_node_graph::InputPort;
use empower_node_graph::Node;
use empower_node_graph::OutputPort;

use crate::graph_editor::GraphEditor;
use crate::graph_editor::display_node::DisplayNode; // @TODO, simplify this include
use crate::graph_editor::display_port::DisplayPort;
use super::GraphViewport;

pub struct NodeWidgetResponse
{
    pub key: EmpowerKey,
    pub kind: NodeWidgetResponseType
}

pub enum NodeWidgetResponseType
{
    ClickedTitle,
    ClickedInputPort(i32), // @TODO, change this to empowerkeys
    ClickedOutputPort(i32),
    ChangedInputPortValueText(i32, String),
}

pub fn show(ui: &mut egui::Ui, graph_editor: &GraphEditor, graph_viewport: &GraphViewport, node_key: &EmpowerKey) -> Option<NodeWidgetResponse>
{
    let mut node_widget_response = None;

    let display_node = graph_editor.display_nodes.get(&node_key).unwrap();

    let debug_mode = true;

    let graph_viewport_title = &graph_viewport.title;

    // @TODO, consider changing this to return a node reponse instead of taking it as input?
    show_node_body(ui, display_node, &graph_editor.selected_nodes, graph_viewport_title, node_key, &mut node_widget_response, &debug_mode);

    let empower_node = graph_editor.empower_node_graph.nodes.get(&node_key).unwrap();
    let input_port_keys = &empower_node.input_port_keys;
    for input_port_key in input_port_keys
    {
        let input_port = graph_editor.empower_node_graph.input_ports.get(input_port_key).unwrap(); 
        let display_input_port = graph_editor.display_input_ports.get(input_port_key).unwrap();

        let port_has_connection = graph_editor.empower_node_graph.connections_in.contains_key(input_port_key);

        show_input_port(ui, display_node, display_input_port, input_port, graph_viewport_title, port_has_connection, node_key, input_port_key, &mut node_widget_response, &debug_mode);
    }

    let output_port_keys = &empower_node.output_port_keys;
    for output_port_key in output_port_keys
    {
        let output_port = graph_editor.empower_node_graph.output_ports.get(&output_port_key).unwrap(); 
        let display_output_port = graph_editor.display_output_ports.get(&output_port_key).unwrap();
        show_output_port(ui, display_node, display_output_port, output_port, graph_viewport_title, node_key, output_port_key, &mut node_widget_response, &debug_mode);
    }

    node_widget_response

}

pub fn show_node_body(ui: &mut egui::Ui, display_node: &DisplayNode, selected_nodes: &Vec<EmpowerKey>, graph_viewport_title: &String, node_key: &EmpowerKey, node_widget_response: &mut Option<NodeWidgetResponse>, debug_mode: &bool)
{
    let node_position = display_node.position;
    let node_screen_size= display_node.size;

    let node_rect= egui::Rect::from_min_size(
        node_position,
        node_screen_size
    );

    let node_body_color = egui::Color32::from_rgb(63, 63, 63);
    let rect_margin = egui::Vec2 { x: 10.0, y: 10.0 };

    let node_outline_rect = node_rect.expand2(rect_margin);

    // @TODO, consider making this a bool that is taken as input to the function instead
    let node_is_selected = selected_nodes.iter().any(| selected_node_key | *selected_node_key == *node_key ); // @TODO, find a reduce the computation of this check
  
    let title_text_font_size = 40.0;
    let mut node_title = display_node.title.clone();

    if *debug_mode
    {
       node_title = format!("{} [{}]", node_title, node_key.to_string());
    }

    let text_size = ui
        .painter()
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

    let node_body_title_area_overlap = 12.0;
    let node_body_bottom_area_overlap = 20.0;

    let node_title_pos= egui::Pos2 {
        x: title_box_rect.center().x,
        y: title_box_rect.min.y
            + (title_box_rect.size().y - node_body_title_area_overlap) / 2.0,
    };

    let node_rect_round_bottom = egui::Rect::from_min_size(
        egui::Pos2 {
            x: node_rect.min.x,
            y: node_rect.max.y - 20.0,
        },
        egui::Vec2 {
            x: node_rect.size().x,
            y: 20.0,
        },
    );

    let node_rect_without_title_and_bottom = egui::Rect::from_min_size(
        egui::Pos2 {
            x: node_rect.min.x,
            y: node_rect.min.y + title_box_rect.size().y - node_body_title_area_overlap,
        },
        egui::Vec2 {
            x: node_rect.size().x,
            y: node_rect.size().y - title_box_rect.size().y - node_rect_round_bottom.size().y
                + node_body_bottom_area_overlap,
        },
    );

    let mut title_rect_color= egui::Color32::from_rgb(50, 50, 50);

    // @TODO, change the name
    let node_reponse = ui.interact(
        title_box_rect,
        egui::Id::new(graph_viewport_title.to_owned() + "_node_body_" + node_key.to_string().as_str()), // @TODO, find a way to move title out of state
        egui::Sense::click_and_drag(),
    );
  
    if node_reponse.hovered() // Important that this is done before clicked
    {
        title_rect_color= egui::Color32::from_rgb(40, 40, 40);
    }

    if node_reponse.clicked()
    {
        *node_widget_response = Some( NodeWidgetResponse { key: *node_key, kind: NodeWidgetResponseType::ClickedTitle });
    }

    if node_is_selected
    {
        ui.painter().rect(
            node_outline_rect,
            6.0,
            egui::Color32::ORANGE,
            egui::Stroke::NONE,
            egui::StrokeKind::Inside,
        );
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
        // egui::Align2::LEFT_TOP,
        egui::Align2::CENTER_CENTER,
        node_title,
        // egui::FontId::monospace(40.0),
        egui::FontId::proportional(title_text_font_size),
        egui::Color32::WHITE,
    );

    // Show node body
    ui.painter().rect(
        node_rect_without_title_and_bottom,
        0.0,
        node_body_color,
        egui::Stroke::NONE,
            egui::StrokeKind::Inside,
    );

    // Show node bottom
    ui.painter().rect(
        node_rect_round_bottom,
        6.0,
        node_body_color,
        egui::Stroke::NONE,
            egui::StrokeKind::Inside,
    );

}

// @TODO, find a way to reduce the number of inputs in this function?
fn show_input_port(ui: &mut egui::Ui, display_node: &DisplayNode, display_port: &DisplayPort, input_port: &InputPort, graph_viewport_title: &String, port_has_coonection: bool, node_key: &EmpowerKey, port_key: &EmpowerKey, node_widget_response: &mut Option<NodeWidgetResponse>, debug_mode: &bool)
{
    let input_port_position = display_node.position + display_port.relative_position;
    let input_port_size = egui::Vec2 { x: 50.0, y: 50.0 }; 

    let input_port_rect = egui::Rect::from_center_size(input_port_position, input_port_size);

    if ui.interact(input_port_rect, egui::Id::from( graph_viewport_title.to_owned() + "_input_port_" + port_key.to_string().as_str()), egui::Sense::click()).clicked()
    {
        *node_widget_response = Some( NodeWidgetResponse { key: *node_key, kind: NodeWidgetResponseType::ClickedInputPort(*port_key) })
    }

    let port_color;
    let port_text;
    match input_port.value 
    {
        EmpowerData::Trigger =>
        {
            port_text = "";
            port_color = egui::Color32::WHITE;
        }
        EmpowerData::Integer(_) =>
        {

            port_text = "value";
            port_color = egui::Color32::YELLOW;
        }
        _ =>
        {
            port_text = "unknown";
            port_color = egui::Color32::YELLOW; 
        }
        
    }

    ui.painter().circle(
        input_port_position,
        25.0,
        port_color,
        egui::Stroke::NONE,
    );

    if *debug_mode
    {
        ui.painter().text(
            input_port_position,
            egui::Align2::CENTER_CENTER,
            port_key.to_string(),
            egui::FontId::proportional(25.0),
            egui::Color32::BLACK,
        );
    }

    let input_port_text_offset = egui::Vec2 { x: 40.0, y: 0.0};
    let input_port_text_position = input_port_position + input_port_text_offset;

    ui.painter().text(
        input_port_text_position,
        // egui::Align2::LEFT_TOP,
        egui::Align2::LEFT_CENTER,
        port_text,
        egui::FontId::proportional(35.0),
        egui::Color32::WHITE,
    );

    match input_port.value // @TODO, find a way to reduce it to one match statement?
    {
        EmpowerData::Trigger =>
        {

        },
        EmpowerData::Integer(_) =>
        {
            let input_port_value_box_position = input_port_text_position + egui::Vec2 { x: 50.0, y: 0.0 };
            let input_port_value_box_size = egui::Vec2{ x: 120.0, y: 40.0 };
            let input_port_value_box_rect = egui::Rect::from_min_size(input_port_value_box_position + egui::Vec2 { x: 50.0, y: -20.0 }, input_port_value_box_size);

            let mut text_edit_color = egui::Color32::WHITE;
            let mut text_background_color = egui::Color32::BLACK;

            if port_has_coonection
            {
                text_edit_color = egui::Color32::GRAY;
                text_background_color = egui::Color32::TRANSPARENT;
            }

            if !display_port.value_text_valid
            {
                text_edit_color = egui::Color32::RED;
            }

            let mut display_port_text = display_port.value.clone();
            let text_edit = egui::TextEdit::singleline(&mut display_port_text)
            .char_limit(6)
            .font(egui::FontId::proportional(35.0))
            .interactive(!port_has_coonection)
            .text_color(text_edit_color)
            .background_color(text_background_color);
            
            ui.put(input_port_value_box_rect, text_edit);

            if display_port_text != display_port.value
            {
                // @TODO, consider how to change this for other than text
                *node_widget_response = Some( NodeWidgetResponse { key: *node_key, kind: NodeWidgetResponseType::ChangedInputPortValueText(*port_key, display_port_text) } );
            }

        },
        _ =>
        {

        } 
    }
}

fn show_output_port(ui: &mut egui::Ui, display_node: &DisplayNode, display_port: &DisplayPort, output_port: &OutputPort, graph_viewport_title: &String, node_key: &EmpowerKey, port_key: &EmpowerKey, node_widget_response: &mut Option<NodeWidgetResponse>, debug_mode: &bool)
{
    let output_port_position = display_node.position + display_port.relative_position;

    let output_port_size = egui::Vec2 { x: 50.0, y: 50.0 }; 
    let output_port_rect= egui::Rect::from_center_size(output_port_position, output_port_size);

    // if ui.interact(output_port_rect, egui::Id::from( graph_title.clone() + "_output_port_" + output_port_key.to_string().as_str()), egui::Sense::click()).clicked()
    if ui.interact(output_port_rect, egui::Id::from( graph_viewport_title.to_owned() + "_output_port_" + port_key.to_string().as_str()), egui::Sense::click()).clicked()
    {
        *node_widget_response = Some( NodeWidgetResponse { key: *node_key, kind: NodeWidgetResponseType::ClickedOutputPort(*port_key) });
        // output_port_interaction(&mut graph_editor.empower_node_graph, graph_viewport, port_key, node_widget_response);
    }

    let port_color;
    match output_port.value 
    {
        EmpowerData::Trigger =>
        {
            port_color = egui::Color32::WHITE;
        }
        _ =>
        {
            port_color = egui::Color32::YELLOW;
        }
    }

    ui.painter().circle(
        output_port_position,
        25.0,
        port_color,
        egui::Stroke::NONE,
    );

    if *debug_mode
    {
        ui.painter().text(
            output_port_position,
            egui::Align2::CENTER_CENTER,
            port_key.to_string(),
            egui::FontId::proportional(25.0),
            egui::Color32::BLACK,
        );
    }
}
