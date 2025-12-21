use empower_engine::NodeGraphKey;
use crate::actions::Action;
use crate::graph_editor::GraphEditor;
use crate::graph_editor::display_node::DisplayValue;

pub fn show_input_port(
                        ui: &mut egui::Ui, 
                        input_port_key: &NodeGraphKey,
                        graph_editor: &mut GraphEditor,
                        graph_viewport_title: &String, 
                        debug_mode: &bool,
                        action_queue: &mut Vec<Action>,
                    )
{
    let port_has_connection = graph_editor.node_graph.input_port_has_connection(input_port_key); // This needs to be placed here for the borrow checker 

    let input_port = graph_editor.node_graph.get_input_port_mut(input_port_key).unwrap();
    let display_input_port = graph_editor.display_input_ports.get_mut(input_port_key).unwrap();

    let input_port_position = display_input_port.position;
    // {
    //     let display_node = graph_editor.display_nodes.get(&input_port.node_key).unwrap();
    //     display_node.position + display_input_port.position.to_vec2()
    // };
    
    // @TODO, make global!
    let input_port_size = egui::Vec2 { x: 50.0, y: 50.0 }; 
 
    let input_port_rect = egui::Rect::from_center_size(display_input_port.position, input_port_size);

    let input_port_response = ui.interact(input_port_rect, egui::Id::from( graph_viewport_title.to_owned() + "_input_port_" + input_port_key.to_string().as_str()), egui::Sense::click());
    if input_port_response.clicked()
    {
        action_queue.push( Action::ClickedInputPort { port_key: *input_port_key });
    }

    input_port_response.on_hover_text( format!("{:?}, {:?}", input_port.value, input_port.compatability ));

    let port_color = display_input_port.color;

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
            input_port_key.to_string(),
            egui::FontId::proportional(25.0),
            egui::Color32::BLACK,
        );
    }
    
    let input_port_text_offset = egui::Vec2 { x: 40.0, y: 0.0};
    let input_port_text_position = input_port_position + input_port_text_offset;

    let input_port_text_font_size = 35.0;

    let painted_text = ui.painter().text(
        input_port_text_position,
        egui::Align2::LEFT_CENTER,
        &display_input_port.text,
        egui::FontId::proportional(input_port_text_font_size),
        egui::Color32::WHITE,
    );

    if port_has_connection
    {
        return;
    }
    
    // @TODO, make this const?
    let text_and_display_value_buffer = 20.0;

    let painted_text_size = painted_text.size();
    let input_port_value_position = input_port_text_position + egui::Vec2 { x: painted_text_size.x + text_and_display_value_buffer, y: -painted_text_size.y / 2.0 };

    let mut potentially_modified_display_value= display_input_port.value.clone(); // @TODO, find a better name
    match &mut potentially_modified_display_value
    {
        DisplayValue::Nothing => {},
        DisplayValue::Text( text ) =>
        {
            let input_port_value_box_size = egui::Vec2{ x: 120.0, y: painted_text_size.y };
            let input_port_value_box_rect = egui::Rect::from_min_size(input_port_value_position, input_port_value_box_size);

            let mut text_edit_color = egui::Color32::WHITE;
            let text_background_color = egui::Color32::BLACK;

            if !display_input_port.valid
            {
                text_edit_color = egui::Color32::RED;
            }

            let text_edit = egui::TextEdit::singleline(text)
            .char_limit(5)
            .font(egui::FontId::proportional(35.0))
            // .interactive(!port_has_connection)
            .text_color(text_edit_color)
            .background_color(text_background_color);

            ui.put(input_port_value_box_rect, text_edit);
        },
        DisplayValue::Checkbox( toggle ) =>
        {
            let input_port_checkbox_size = egui::Vec2{ x: 120.0, y: 0.0 };
            let input_port_checkbox_rect = egui::Rect::from_min_size(input_port_value_position, input_port_checkbox_size);

            // @TODO, improve this, and fix box size
            let checkbox = egui::Checkbox::new(
                                            toggle, 
                                            egui::RichText::new("").font(egui::FontId::proportional(35.0)));
            ui.put(input_port_checkbox_rect, checkbox);
        },
    }

    if potentially_modified_display_value != display_input_port.value
    {
        graph_editor.set_input_port_value_if_display_value_can_convert(input_port_key, &potentially_modified_display_value);
    } 
}
