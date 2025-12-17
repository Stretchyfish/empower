
use empower_engine::{PortValue, node_graph::node::NodeKind};

use crate::graph_editor::DisplayPort;

use super::DisplayNodeKind;

#[derive(Clone)]
pub struct ShowImageDisplayNode
{

}

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
                                                        egui::pos2(0.0, 0.0), 
                                                        &input_port_values[0]
                                                    );
        display_inputs.push(display_port_trigger);

        let display_port_x = DisplayPort::new(
                                                        "file path".to_string(), 
                                                        egui::pos2(0.0, 0.0), 
                                                        &input_port_values[1]
                                                    );
        display_inputs.push(display_port_x);

        display_inputs
    }

    fn state_size(&self) -> egui::Vec2 {
        egui::Vec2 { x: 0.0, y: 0.0 }
    }

    fn state_show(&mut self, _: &mut egui::Ui, _: &mut Box<dyn NodeKind>) -> bool {
        false
    }
}


