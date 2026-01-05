use empower_engine::PortValue;
use empower_engine::node_graph::node::NodeKind;
use super::DisplayNodeKind;
use super::super::DisplayPort;
use empower_engine::node_graph::node::node_kind::{ConditionNode, ConditionType};

#[derive(Clone)]
pub struct ConditionDisplayNode
{

}

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
        egui::Vec2 { x: 350.0, y: 350.0 }
    }

    fn state_size(&self) -> egui::Vec2 {
        egui::Vec2 { x: 175.0, y: 50.0 }
    }

    fn display_input_ports(&self, input_port_values: Vec<&PortValue>) -> Vec<DisplayPort> {

        let mut display_inputs = Vec::new();

        display_inputs.reserve(input_port_values.len());
        for input_port_value in input_port_values
        {
            let display_port = DisplayPort::new(
                                                "A".to_string(), 
                                                egui::pos2(0.0, 0.0), 
                                                &input_port_value
                                            );
            display_inputs.push(display_port);
        }

        display_inputs
    }

    fn display_output_ports(&self, output_port_values: Vec<&PortValue>) -> Vec<DisplayPort> {

        vec![
            DisplayPort::new(String::from("true"), egui::pos2(0.0, 0.0), output_port_values[0]),
            DisplayPort::new(String::from("false"), egui::pos2(0.0, 0.0), output_port_values[1]),
        ]
    }

    fn state_show(&mut self, ui: &mut egui::Ui, node_kind: &mut Box<dyn NodeKind>) -> bool {

        let condition_node_state = node_kind.as_any_mut().downcast_mut::<ConditionNode>().expect("Condition display node tried to unwrap a node_kind that is not the condition node kind");

        let original_condition_type = condition_node_state.condition_type.clone();

        ui.menu_button(condition_node_state.condition_type.to_string(), |ui|
        {
            if ui.button("Equal").clicked()
            {
                condition_node_state.condition_type = ConditionType::Equal;
            }
            if ui.button("Greater").clicked()
            {
                condition_node_state.condition_type = ConditionType::Greater;
            }
            if ui.button("Less").clicked()
            {
                condition_node_state.condition_type = ConditionType::Less;
            }
        });

        if original_condition_type != condition_node_state.condition_type
        {
            return true;
        }
        
        false
    }
}
