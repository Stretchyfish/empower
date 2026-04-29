use empower_engine::{PortValue, node_graph::{Variables, node::NodeKind}};

use crate::studio_context::project::graph_editor::{DisplayPort, display_node::DisplayNodeStateResponse};

use super::DisplayNodeKind;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct RangeDisplayNode
{
    
}

#[typetag::serde]
impl DisplayNodeKind for RangeDisplayNode
{
    fn new() -> Box<dyn DisplayNodeKind>where Self:Sized {

        Box::new( Self {} )
    }

    fn clone_box(&self) -> Box<dyn DisplayNodeKind>  {

        Box::new( self.clone() )
    }

    fn node_size(&self, _: &Box<dyn NodeKind>) -> egui::Vec2 {

        egui::Vec2 { x: 350.0, y: 300.0 }
    }

    fn display_input_ports(&self,input_port_values:Vec<&PortValue>) -> Vec<DisplayPort>  {
        vec![
            DisplayPort::new(
                        "A".to_string(), 
                        egui::vec2(0.0, 0.0), 
                        &input_port_values[0]
                    ),
            DisplayPort::new(
                        "B".to_string(), 
                        egui::vec2(0.0, 0.0), 
                        &input_port_values[1]
                    ),
            DisplayPort::new(
                        "C".to_string(), 
                        egui::vec2(0.0, 0.0), 
                        &input_port_values[2]
                    )
        ]
    }

    fn display_output_ports(&self,output_port_values:Vec< &PortValue>) -> Vec<DisplayPort>  {
        vec![
            DisplayPort::nothing(egui::vec2(0.0, 0.0), &output_port_values[0])
        ]
    }

    fn state_size(&self) -> egui::Vec2 {
        egui::Vec2::ZERO
    }

    fn state_show(&mut self, _: &mut egui::Ui, _: &mut Box<dyn NodeKind>, _: &Variables) -> DisplayNodeStateResponse {
        DisplayNodeStateResponse::NoChange
    }
}
