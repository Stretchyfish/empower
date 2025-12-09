use std::fmt;

use crate::node_graph::node::{NodeFunction, port::PortCompatability, port::PortValue};

use super::NodeKind;

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

    fn as_any(&mut self) -> &mut dyn std::any::Any {
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

    fn setup(&mut self, inputs: Vec<&PortValue>) -> Option<Vec<PortValue>> {
        Some( Vec::from( [ inputs[0].clone() ] ))
    }

    fn update(&mut self) -> Option<Vec<PortValue>> {
        todo!()
    }

    fn execute(&mut self, _: &mut egui::Ui) {
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
