use egui;

pub fn get_display_start_node_size() -> egui::Vec2
{
    egui::Vec2 { x: 250.0, y: 165.0 }
}

pub fn get_display_start_node_input_port_names() -> Vec<String>
{
    Vec::new()
}

pub fn get_display_start_node_output_ports_names() -> Vec<String>
{
    vec![ "".to_string() ]
}
