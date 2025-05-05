use crate::viewports::graph_viewport::{graph_viewport_state, GraphViewportState};
use crate::viewports::graph_viewport::utils::PortKind;
use crate::NodeGraph;
use crate::interactions::user::UserInputs;

pub fn view_connection_search(ui: &mut egui::Ui, node_graph: &mut NodeGraph, graph_viewport_state: &GraphViewportState, user_input: &UserInputs)
{
    if graph_viewport_state.port_search.is_none()
    {
        return;
    }

    let port_search = graph_viewport_state.port_search.unwrap();

    let node_key;
    let port_relative_position;
    match port_search.port_kind
    {
        PortKind::InputPort => 
        {
            let display_input_port = node_graph.display_input_ports.get(&port_search.port_key).unwrap();
            port_relative_position = display_input_port.relative_position;                   
            node_key = display_input_port.node_key;
        }

        PortKind::OutputPort =>
        {
            let display_output_port= node_graph.display_output_ports.get(&port_search.port_key).unwrap();
            port_relative_position = display_output_port.relative_position;                   
            node_key = display_output_port.node_key;
        }
    }

    // @TODO A crash can happen here if the node gets removed, needs to be fixed
    let display_node = node_graph.display_nodes.get(&node_key).unwrap();
    // let node_position = node_graph.display_nodes.get(&node_key).unwrap().position;

    let node_centering_offset = egui::Vec2{ x: -display_node.size.x / 2.0 , y: 0.0 }; // This value is used to center node around its middle, instead of around its top left corner
    let port_position = display_node.position + port_relative_position + node_centering_offset;
    // let port_position = node_position;

    let port_draw_position = graph_viewport_state.pan_zoom.world_to_screen(&port_position);

    ui.painter().line_segment([ port_draw_position, user_input.mouse_position], egui::Stroke::new(5.0 * graph_viewport_state.pan_zoom.zoom_scale, egui::Color32::YELLOW));
}
