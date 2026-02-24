use empower_engine::node_graph::node::node_kind::{NumberNode, NumberNodeValueKind};
use empower_engine::node_graph::node::NodeKind;

use super::DisplayNodeKind;
use crate::studio_context::project::graph_editor::display_node::DisplayNodeStateResponse;
use crate::studio_context::project::graph_editor::DisplayPort;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct NumberDisplayNode
{

}

#[typetag::serde]
impl DisplayNodeKind for NumberDisplayNode
{
    fn new() -> Box<dyn DisplayNodeKind> where
        Self: Sized {
        
        Box::new( Self {} )
    }

    fn clone_box(&self) -> Box<dyn DisplayNodeKind> {

        Box::new( self.clone() )
    }

    fn node_size(&self, _: &Box<dyn NodeKind>) -> egui::Vec2 {
        egui::Vec2 { x: 320.0, y: 210.0 }
     }

    fn display_input_ports(&self, input_port_values: Vec<&empower_engine::PortValue>) -> Vec<DisplayPort> {
        vec![
            DisplayPort::new(
                                                            "A".to_string(), 
                                                            egui::vec2(0.0, 0.0), 
                                                            &input_port_values[0]
                                                        )
        ]
    }

    fn display_output_ports(&self, output_port_values: Vec<&empower_engine::PortValue>) -> Vec<DisplayPort> {
        
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
        egui::Vec2 { x: 175.0, y: 50.0 }
    }

    fn state_show(&mut self, ui: &mut egui::Ui, node_kind: &mut Box<dyn NodeKind>) -> DisplayNodeStateResponse {

        let number_node_state = node_kind.as_any_mut().downcast_mut::<NumberNode>().expect("Number display node tried to unwrap a node_kind that is not the number node kind");

        let original_desired_value = number_node_state.desired_value.clone();

        ui.menu_button(number_node_state.desired_value.to_string(), |ui|
        {
            if ui.button("Automatic").clicked()
            {
                number_node_state.desired_value = NumberNodeValueKind::Automatic;
            }
            if ui.button("Integer").clicked()
            {
                number_node_state.desired_value = NumberNodeValueKind::Integer;
            }
            if ui.button("Float").clicked()
            {
                number_node_state.desired_value = NumberNodeValueKind::Float;
            }
        });

        if original_desired_value != number_node_state.desired_value
        {
            return DisplayNodeStateResponse::RefreshNodeStructure;
        }

        DisplayNodeStateResponse::NoChange
    }
} 
