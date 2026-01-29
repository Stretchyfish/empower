use std::fmt;

use crate::node_graph::node::port::{PortCompatability, PortValue};

use super::NodeKind;
use super::NodeSetupResponse;
use super::NodeUpdateResponse;

#[derive(Clone)]
pub struct NumberNode
{
    pub desired_value: NumberNodeValueKind,
}

impl NodeKind for NumberNode
{
    fn new() -> Box<dyn NodeKind> where
        Self: Sized {

        Box::new(
            Self {
                desired_value: NumberNodeValueKind::Automatic,
            }
        )
    }

    fn name(&self) -> &'static str {
        "number"
    }

    fn clone_box(&self) -> Box<dyn NodeKind> {
        Box::new(self.clone())
    }

    fn input_compatabilities(&self) -> Vec<PortCompatability> {

        match self.desired_value
        {
            NumberNodeValueKind::Automatic => Vec::from( [ PortCompatability::OneOf( vec!( PortValue::Integer(0), PortValue::Float(0.0)  ) ) ]),
            NumberNodeValueKind::Integer => Vec::from( [ PortCompatability::Exatch( PortValue::Integer(0) ) ]),
            NumberNodeValueKind::Float => Vec::from( [ PortCompatability::Exatch( PortValue::Float(0.0) ) ]),
        }
    }

    fn output_compatabilities(&self) -> Vec<PortCompatability> {

        match self.desired_value
        {
            NumberNodeValueKind::Automatic => Vec::from( [ PortCompatability::OneOf( vec!( PortValue::Integer(0), PortValue::Float(0.0)  ) ) ]),
            NumberNodeValueKind::Integer => Vec::from( [ PortCompatability::Exatch( PortValue::Integer(0) ) ]),
            NumberNodeValueKind::Float => Vec::from( [ PortCompatability::Exatch( PortValue::Float(0.0) ) ]),
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn setup(&mut self, inputs: Vec<&PortValue>) -> NodeSetupResponse {
        NodeSetupResponse::Finished(  vec![ inputs[0].clone() ] )
    }

    fn update(&mut self) -> NodeUpdateResponse {
        todo!()
    }

    fn show(&mut self, _: &mut egui::Ui) {
        todo!()
    }
}

#[derive(Default, Clone, PartialEq, Eq, Debug)]
pub enum NumberNodeValueKind
{
    #[default] Automatic,
    Integer,
    Float,
}

impl fmt::Display for NumberNodeValueKind
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result 
    {
        write!(f, "{:?}", self)
    }
}
