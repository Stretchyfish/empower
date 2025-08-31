use empower_engine::NodeKind;

pub mod display_start_node;

pub fn get_display_node_size(node_kind: &NodeKind) -> egui::Vec2
{
    match node_kind
    {
        NodeKind::Start => display_start_node::get_display_start_node_size(),
        _ => egui::Vec2::ZERO,
    }
}

pub fn get_display_node_value_offset(node_kind: &NodeKind) -> f32
{
    match node_kind
    {
        _ => 0.0,
    }
} 

pub fn get_display_input_port_names(node_kind: &NodeKind) -> Vec<String>
{
    match node_kind
    {
        NodeKind::Start => display_start_node::get_display_start_node_input_port_names(),
        _ => Vec::new(),
    }
}

pub fn get_display_output_port_names(node_kind: &NodeKind) -> Vec<String>
{
    match node_kind
    {
        NodeKind::Start => display_start_node::get_display_start_node_output_ports_names(),
        _ => Vec::new(),
    }
}