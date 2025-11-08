use std::fmt;

use crate::node_graph::node::{NodeFunction, port::PortCompatability, port::PortValue};

use super::NodeKind;

#[derive(Clone)]
pub struct NumberNode
{
    pub desired_value: NumberNodeValueKind,
    port_value_to_send: PortValue,
}

impl NodeKind for NumberNode
{
    fn new() -> Box<dyn NodeKind> where
        Self: Sized {

        Box::new(
            Self {
                desired_value: NumberNodeValueKind::Automatic,
                port_value_to_send: PortValue::Integer(0),
            }
        )
    }

    fn name(&self) -> &'static str {
        "number"
    }

    fn clone_box(&self) -> Box<dyn NodeKind> {
        Box::new(self.clone())
    }

    fn function(&self) -> NodeFunction {
        NodeFunction::Instant
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

    fn state(&mut self, ui: &mut egui::Ui) {
        
        ui.menu_button(self.desired_value.to_string(), |ui|
        {
            if ui.button("Automatic").clicked()
            {
                self.desired_value = NumberNodeValueKind::Automatic;
            }
            if ui.button("Integer").clicked()
            {
                self.desired_value = NumberNodeValueKind::Integer;
            }
            if ui.button("Float").clicked()
            {
                self.desired_value = NumberNodeValueKind::Float;
            }
        });
    }

    fn setup(&mut self, inputs: Vec<&PortValue>) {
        self.port_value_to_send = inputs[0].clone();
    }

    fn update(&mut self, _: Option<&mut egui::Ui>) -> Option<Vec<PortValue>> {
        Some( Vec::from( [ self.port_value_to_send.clone() ] ))
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
