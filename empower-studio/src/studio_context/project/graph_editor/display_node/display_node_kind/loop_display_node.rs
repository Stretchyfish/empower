use std::fmt::Debug;

use empower_engine::node_graph::Variables;
use empower_engine::node_graph::node::node_kind::{LoopNode, LoopType, NumberNode, NumberNodeValueKind};
use empower_engine::node_graph::node::NodeKind;

use super::DisplayNodeKind;
use crate::studio_context::project::graph_editor::display_node::DisplayNodeStateResponse;
use crate::studio_context::project::graph_editor::DisplayPort;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct LoopDisplayNode
{
}

#[typetag::serde]
impl DisplayNodeKind for LoopDisplayNode
{
    fn new() -> Box<dyn DisplayNodeKind> where
        Self: Sized {
        
        Box::new(
            Self
            {
            }
        )
    }

    fn clone_box(&self) -> Box<dyn DisplayNodeKind> {

        Box::new( self.clone() )
    }

    fn node_size(&self, node_kind: &Box<dyn NodeKind>) -> egui::Vec2 {

        let loop_node_state = node_kind.as_any().downcast_ref::<LoopNode>().expect("loop display node tried to unwrap a node_kind that is not the loop node kind");

        let n = loop_node_state.input_compatabilities().len();
        let height = 300.0 + 70.0 * (n as f32 - 2.0); // @TODO, distance between nodes needs to be a global value
    
        egui::Vec2 { x: 450.0, y: height }
     }

    fn display_input_ports(&self, input_port_values: Vec<&empower_engine::PortValue>) -> Vec<DisplayPort> {

        let mut display_outputs = Vec::new();

        display_outputs.reserve(input_port_values.len());
        for input_port_value in input_port_values
        {
            let display_port = DisplayPort::nothing(egui::vec2(0.0, 0.0), &input_port_value); // The position is just defaulted here, because it will be correct in refresh display node
            display_outputs.push(display_port);
        }

        display_outputs
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

    fn state_show(&mut self, ui: &mut egui::Ui, node_kind: &mut Box<dyn NodeKind>, _: &Variables) -> DisplayNodeStateResponse {

        let loop_node_state = node_kind.as_any_mut().downcast_mut::<LoopNode>().expect("Loop display node tried to unwrap a node_kind that is not the loop node kind");

        let original_type = loop_node_state.loop_type.clone();

        ui.menu_button(format!("{:?}", original_type), |ui|
        {
            if ui.button("Forever").clicked()
            {
                loop_node_state.loop_type = LoopType::Forever;
            }
            if ui.button("Range").clicked()
            {
                loop_node_state.loop_type = LoopType::Range;
            }
        });

        if original_type != loop_node_state.loop_type
        {
            return DisplayNodeStateResponse::RefreshNodeStructure;
        }

        DisplayNodeStateResponse::NoChange
    }
} 

