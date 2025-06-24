use crate::viewports::graph_viewport::GraphViewportState;
use super::PortKind;
use crate::{studio_context, StudioContext};
use crate::interactions::user::UserInputs; // @TODO, clean up this inputs

pub fn view_connection_search(ui: &mut egui::Ui, studio_context: &mut StudioContext, graph_viewport_state: &GraphViewportState, mouse_scene_position: &egui::Pos2)
{
    if graph_viewport_state.port_search.is_none()
    {
        // println!("Not searching for a connection");
        return;
    }

    let port_search = graph_viewport_state.port_search.unwrap();

    let node_key;
    let port_relative_position;
    match port_search.port_kind
    {
        PortKind::InputPort => 
        {
            let display_input_port = studio_context.display_node_graph.display_input_ports.get(&port_search.port_key).unwrap(); // @TODO, simplify this call
            port_relative_position = display_input_port.relative_position;                   
            node_key = display_input_port.node_key;
        }

        PortKind::OutputPort =>
        {
            let display_output_port= studio_context.display_node_graph.display_output_ports.get(&port_search.port_key).unwrap(); // @TODO, simplify this call
            port_relative_position = display_output_port.relative_position;                   
            node_key = display_output_port.node_key;
        }
    }

    // @TODO A crash can happen here if the node gets removed, needs to be fixed
    let display_node = studio_context.display_node_graph.display_nodes.get(&node_key).unwrap(); // @TODO, simplify this call
    // let node_position = node_graph.display_nodes.get(&node_key).unwrap().position;

    let node_centering_offset = egui::Vec2{ x: -display_node.size.x / 2.0 , y: 0.0 }; // This value is used to center node around its middle, instead of around its top left corner
    // let port_position = display_node.position + port_relative_position + node_centering_offset;
    let port_position = display_node.position + port_relative_position;
    // let port_position = node_position;

    // let port_draw_position = graph_viewport_state.pan_zoom.world_to_screen(&port_position);

    ui.painter().line_segment([ port_position, *mouse_scene_position], egui::Stroke::new(10.0, egui::Color32::YELLOW));
}
