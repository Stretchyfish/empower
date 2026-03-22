use std::collections::HashMap;

use empower_engine::{PortValue, node_graph::Variable};
use empower_engine::node_graph::node::NodeKind;
use crate::studio_context::project::graph_editor::display_node::DisplayNodeStateResponse;

use super::DisplayNodeKind;
use super::super::DisplayPort;
use empower_engine::node_graph::node::node_kind::{ConditionNode, ConditionType, VariableNode};

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct VariableDisplayNode
{
    
}

#[typetag::serde]
impl DisplayNodeKind for VariableDisplayNode
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
        Vec::new()
    }

    fn display_output_ports(&self, output_port_values: Vec<&PortValue>) -> Vec<DisplayPort> {
        Vec::new()
    }
    
    fn state_show(&mut self, ui: &mut egui::Ui, node_kind: &mut Box<dyn NodeKind>, variables: &HashMap<String, Variable>) -> DisplayNodeStateResponse {


        let number_node_state = node_kind.as_any_mut().downcast_mut::<VariableNode>().expect("Variable display node tried to unwrap a node_kind that is not the variable node kind");

        let current_variable_name = match number_node_state.variable_key.as_ref()
        {
            Some( variable_name ) => variable_name.clone(),
            None => String::from("None"),
        };

        ui.menu_button( current_variable_name, |ui|
        {
            for (variable_name, _) in variables
            {
                if ui.button(variable_name).clicked()
                {
                    number_node_state.variable_key = Some( variable_name.clone() );
                }
            }
        });
        
        DisplayNodeStateResponse::NoChange
    }
}
