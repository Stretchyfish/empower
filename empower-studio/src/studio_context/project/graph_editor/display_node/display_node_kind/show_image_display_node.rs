use empower_engine::node_graph::Variables;
use empower_engine::{PortValue, node_graph::node::NodeKind};

use crate::studio_context::project::graph_editor::display_node::DisplayNodeStateResponse;
use crate::studio_context::project::graph_editor::DisplayPort;

use super::DisplayNodeKind;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct ShowImageDisplayNode
{

}

#[typetag::serde]
impl DisplayNodeKind for ShowImageDisplayNode
{
    fn new() -> Box<dyn DisplayNodeKind> where
        Self: Sized {
        
        Box::new( Self {} )
    }

    fn clone_box(&self) -> Box<dyn DisplayNodeKind> {

        Box::new( self.clone() )
    }

    fn node_size(&self, _: &Box<dyn NodeKind>) -> egui::Vec2 {
        egui::Vec2 { x: 350.0, y: 230.0 }
    }

    fn display_input_ports(&self, input_port_values: Vec<&PortValue>) -> Vec<DisplayPort> {

        let mut display_inputs = Vec::new();

        display_inputs.reserve(input_port_values.len());

        let display_port_trigger = DisplayPort::nothing(
                                                        egui::vec2(0.0, 0.0), 
                                                        &input_port_values[0]
                                                    );
        display_inputs.push(display_port_trigger);

        let display_port_x = DisplayPort::new(
                                                        "file path".to_string(), 
                                                        egui::vec2(0.0, 0.0), 
                                                        &input_port_values[1]
                                                    );
        display_inputs.push(display_port_x);

        display_inputs
    }

    fn display_output_ports(&self, output_port_values: Vec<&PortValue>) -> Vec<DisplayPort> {

        let mut display_outputs = Vec::new();

        display_outputs.reserve(output_port_values.len());
        for output_port_value in output_port_values
        {
            let display_port = DisplayPort::nothing(egui::vec2(0.0, 0.0), &output_port_value); // The position is just defaulted here, because it will be correct in refresh display node
            display_outputs.push(display_port);
        }

        display_outputs
    }

    fn state_size(&self) -> egui::Vec2 {
        egui::Vec2 { x: 0.0, y: 0.0 }
    }

    fn state_show(&mut self, _: &mut egui::Ui, _: &mut Box<dyn NodeKind>, _: &Variables) -> DisplayNodeStateResponse {
        DisplayNodeStateResponse::NoChange
    }
}


