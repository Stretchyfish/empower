use std::collections::HashMap;
use std::sync::Arc;

use empower_engine::node_graph::Variables;
use empower_engine::{PortValue, node_graph::Variable};
use empower_engine::node_graph::node::NodeKind;
use crate::studio_context::project::graph_editor::display_node::DisplayNodeStateResponse;

use super::DisplayNodeKind;
use super::super::DisplayPort;
use empower_engine::node_graph::node::node_kind::{ConditionNode, ConditionType};

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct ConditionDisplayNode
{

}

#[typetag::serde]
impl DisplayNodeKind for ConditionDisplayNode
{
    fn new() -> Box<dyn DisplayNodeKind> where
        Self: Sized {
        
        Box::new( Self {} )
    }

    fn clone_box(&self) -> Box<dyn DisplayNodeKind> {
        Box::new( self.clone() )
    }

    fn node_size(&self, _: &Box<dyn NodeKind>) -> egui::Vec2 {
        egui::Vec2 { x: 350.0, y: 360.0 }
    }

    fn state_size(&self) -> egui::Vec2 {
        egui::Vec2 { x: 300.0, y: 60.0 }
    }

    fn display_input_ports(&self, input_port_values: Vec<&PortValue>) -> Vec<DisplayPort> {
        vec![
            DisplayPort::new("".to_string(), egui::vec2(0.0, 0.0), &input_port_values[0]),
            DisplayPort::new("A".to_string(), egui::vec2(0.0, 0.0), &input_port_values[1]),
            DisplayPort::new("B".to_string(), egui::vec2(0.0, 0.0), &input_port_values[2])
        ]
    }

    fn display_output_ports(&self, output_port_values: Vec<&PortValue>) -> Vec<DisplayPort> {

        vec![
            DisplayPort::new(String::from("true"), egui::vec2(0.0, 0.0), output_port_values[0]),
            DisplayPort::new(String::from("false"), egui::vec2(0.0, 0.0), output_port_values[1]),
        ]
    }

    fn state_show(&mut self, ui: &mut egui::Ui, node_kind: &mut Box<dyn NodeKind>, _: &Variables) -> DisplayNodeStateResponse {

        let condition_node_state = node_kind.as_any_mut().downcast_mut::<ConditionNode>().expect("Condition display node tried to unwrap a node_kind that is not the condition node kind");

        let original_condition_type = condition_node_state.condition_type.clone();

        let button_text = match condition_node_state.condition_type
        {
            ConditionType::Equal => "A equal to B",
            ConditionType::GreaterThan => "A greater than B",
            ConditionType::LessThan => "A less than B",
        };

        ui.menu_button(button_text, |ui|
        {
            if ui.button("A equal to B").clicked()
            {
                condition_node_state.condition_type = ConditionType::Equal;
            }
            if ui.button("A greater than B").clicked()
            {
                condition_node_state.condition_type = ConditionType::GreaterThan;
            }
            if ui.button("A less than B").clicked()
            {
                condition_node_state.condition_type = ConditionType::LessThan;
            }
        });

        if original_condition_type != condition_node_state.condition_type
        {
            return DisplayNodeStateResponse::RefreshNodeStructure;
        }
        
        DisplayNodeStateResponse::NoChange
    }
}
