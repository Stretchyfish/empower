use std::any::Any;
use std::fmt;
use std::sync::{Arc, Mutex};

use crate::node_graph::node::port::{PortCompatability, PortValue};
use crate::node_graph::Variable;

use super::NodeKind;
use super::NodeSetupResponse;
use super::NodeUpdateResponse;
use super::NodeResponse;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct VariableNode
{
    pub variable: Option<Arc<Mutex<Variable>>>,
    pub value_mappings: Vec<String>, // This is mainly used to handle the order of values, that the order stays consistent
    pub access_type: AccessType,
}

#[typetag::serde]
impl NodeKind for VariableNode
{
    fn new() -> Box<dyn NodeKind>
        where Self:Sized
        {
            Box::new( Self
            {
                variable: None,
                value_mappings: Vec::new(),
                access_type: AccessType::WriteOnly,
            }
        )
    }

    fn name(&self) ->  &'static str {
        "variable"
    }

    fn clone_box(&self) -> Box<dyn NodeKind>  {
        Box::new( self.clone() )
    }

    fn input_compatabilities(&self) -> Vec<PortCompatability>  {

        if self.variable.is_none()
        {
            return Vec::new();
        }

        if self.access_type == AccessType::WriteOnly
        {
            return Vec::new();
        }

        let variable = self.variable.as_ref().unwrap().lock().unwrap();

        let mut input_port_compatabilities = Vec::new();
        for mapping in &self.value_mappings
        {
            let value = variable.values.get(mapping).unwrap(); // This is dangerous, but should be safe
            let port_compatibility = PortCompatability::Exatch(value.clone());

            input_port_compatabilities.push( port_compatibility );
        }
        
        input_port_compatabilities
    }

    fn output_compatabilities(&self) -> Vec<PortCompatability>  {

        if self.variable.is_none()
        {
            return Vec::new();
        }

        if self.access_type == AccessType::ReadOnly
        {
            return Vec::new();
        }

        let variable = self.variable.as_ref().unwrap().lock().unwrap();

        let mut output_port_compatabilities = Vec::new();
        for mapping in &self.value_mappings
        {
            let value = variable.values.get(mapping).unwrap(); // This is dangerous, but should be safe
            let port_compatibility = PortCompatability::Exatch(value.clone());

            output_port_compatabilities.push( port_compatibility );
        }
        
        output_port_compatabilities
    }

    fn as_any_mut(&mut self) ->  &mut dyn Any {
        self
    }

    fn as_any(&self) ->  &dyn Any {
        self
    }

    fn setup(&mut self, inputs: Vec<&PortValue>) -> NodeResponse {

        if self.variable.is_none()
        {
            return NodeResponse::Finished( Vec::new() );
        }

        let variable = self.variable.as_mut().unwrap();

        // @TODO, this indexing is rather unsafe

        let mut outputs = Vec::new();
        match self.access_type
        {
            AccessType::ReadOnly =>
            {
                for (index, name) in self.value_mappings.iter().enumerate()
                {
                    let input = inputs[index]; 
                    variable.lock().unwrap().values.insert(name.clone(), input.clone()); // @TODO, feels like some of these clone could get removed
                }
            },
            AccessType::WriteOnly =>
            {
                for name in &self.value_mappings
                {
                    let value = variable.lock().unwrap().values.get(name).unwrap().clone();
                    outputs.push( value.clone() );
                }
            },
            AccessType::ReadAndSet =>
            {
                for (index, name) in self.value_mappings.iter().enumerate()
                {
                    let input = inputs[index]; 
                    variable.lock().unwrap().values.insert(name.clone(), input.clone()); // @TODO, feels like some of these clone could get removed
                    let value = variable.lock().unwrap().values.get(name).unwrap().clone();

                    outputs.push( value );
                }
            },
        }

        NodeResponse::Finished( outputs )
    }

    fn update(&mut self) -> NodeResponse {
        todo!()
    }

    fn show(&mut self, _: &mut egui::Ui) {
        todo!()
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub enum AccessType
{
    ReadOnly,
    WriteOnly,
    ReadAndSet
}

impl fmt::Display for AccessType
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        match self
        {
            AccessType::ReadOnly => write!(f, "read only"),
            AccessType::WriteOnly => write!(f, "write only"),
            AccessType::ReadAndSet => write!(f, "read and write"),
        }

    }
}
