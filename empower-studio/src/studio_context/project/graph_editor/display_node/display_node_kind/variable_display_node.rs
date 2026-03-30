use std::sync::{Arc, Mutex};

use empower_engine::node_graph::Variables;
use empower_engine::{PortValue, node_graph::Variable};
use empower_engine::node_graph::node::NodeKind;
use crate::studio_context::project::graph_editor::display_node::DisplayNodeStateResponse;

use super::DisplayNodeKind;
use super::super::DisplayPort;
use empower_engine::node_graph::node::node_kind::VariableNode;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct VariableDisplayNode
{
    pub value_mappings: Vec<String>,
}

#[typetag::serde]
impl DisplayNodeKind for VariableDisplayNode
{
    fn new() -> Box<dyn DisplayNodeKind> where
        Self: Sized {
        
        Box::new( Self {
            value_mappings: Vec::new(),
        } )
    }

    fn clone_box(&self) -> Box<dyn DisplayNodeKind> {
        Box::new( self.clone() )
    }

    fn node_size(&self, _: &Box<dyn NodeKind>) -> egui::Vec2 {

        let n = self.value_mappings.len();
        let height = 350.0 + 70.0 * (n as f32 - 2.0); // @TODO, distance between nodes needs to be a global value
    
        egui::Vec2 { x: 450.0, y: height }
    }

    fn state_size(&self) -> egui::Vec2 {
        egui::Vec2 { x: 300.0, y: 120.0 }
    }

    fn display_input_ports(&self, input_port_values: Vec<&PortValue>) -> Vec<DisplayPort> {

        let mut display_outputs = Vec::new();

        display_outputs.reserve(input_port_values.len());
        for (index, input_port_value) in input_port_values.iter().enumerate()
        {
            let port_name = self.value_mappings[index].clone();
            let display_port = DisplayPort::new(port_name, egui::vec2(0.0, 0.0), &input_port_value); // The position is just defaulted here, because it will be correct in refresh display node
            display_outputs.push(display_port);
        }

        display_outputs
    }

    fn display_output_ports(&self, output_port_values: Vec<&PortValue>) -> Vec<DisplayPort> {

        let mut display_outputs = Vec::new();

        display_outputs.reserve(output_port_values.len());
        for (_, output_port_value) in output_port_values.iter().enumerate()
        {
            let display_port = DisplayPort::nothing(egui::vec2(0.0, 0.0), &output_port_value); // The position is just defaulted here, because it will be correct in refresh display node
            display_outputs.push(display_port);
        }

        println!("Size: {}", display_outputs.len());
        display_outputs
    }
    
    fn state_show(&mut self, ui: &mut egui::Ui, node_kind: &mut Box<dyn NodeKind>, variables: &Variables) -> DisplayNodeStateResponse {

        let variable_node = node_kind.as_any_mut().downcast_mut::<VariableNode>().expect("Variable display node tried to unwrap a node_kind that is not the variable node kind");

        let current_variable_name = match variable_node.variable.as_ref()
        {
            Some( variable ) => variable.lock().unwrap().name.clone(),
            None => String::from("None"),
        };

        let mut variable_changed = false;
        let mut read_access_change = false;

        ui.horizontal(|ui|
        {
            ui.label("variable: ");
            ui.menu_button( current_variable_name, |ui|
            {
                for (variable_name, variable ) in variables
                {
                    if variable_node.variable.is_some() 
                    {
                        if ui.button("none").clicked()
                        {
                            self.remove_variable(variable_node);
                            variable_changed = true;
                        }
                    }
                
                    if ui.button(variable_name).clicked()
                    {
                        self.set_new_variable( variable.clone(), variable_node );
                        variable_changed = true;
                    }
                }
            });

        });

        ui.horizontal(|ui|
        {
            ui.label("access");
            
            ui.menu_button(variable_node.access_type.to_string(), |ui|
            {
                if ui.button("write only").clicked()
                {
                    variable_node.access_type = empower_engine::node_graph::node::node_kind::AccessType::WriteOnly;
                    read_access_change = true;
                }

                if ui.button("read only").clicked()
                {
                    variable_node.access_type = empower_engine::node_graph::node::node_kind::AccessType::ReadOnly;
                    read_access_change = true;
                }

                if ui.button("both").clicked()
                {
                    variable_node.access_type = empower_engine::node_graph::node::node_kind::AccessType::ReadAndSet;
                    read_access_change = true;
                }
            });
        });

        if variable_changed || read_access_change
        {
            return DisplayNodeStateResponse::RefreshNodeStructure;
        }

        DisplayNodeStateResponse::NoChange
    }
}

impl VariableDisplayNode
{
    pub fn set_new_variable(&mut self, new_variable: Arc<Mutex<Variable>>, variable_node: &mut VariableNode)
    {
        let mut ordered_values = Vec::new(); // To make sure the mappings and the values in the node is compatible

        self.value_mappings.clear();
        for( value_name, value ) in &new_variable.lock().unwrap().values
        {
            self.value_mappings.push( value_name.clone() );
            ordered_values.push( value.clone() );
        }

        variable_node.variable = Some( new_variable );
        variable_node.value_mappings = self.value_mappings.clone();
    }

    pub fn remove_variable(&mut self, variable_node: &mut VariableNode)
    {
        self.value_mappings.clear(); // @TODO, this double value mapping approach is not so great, needs to be improved
        variable_node.value_mappings.clear();
        variable_node.variable = None;
    }
}
