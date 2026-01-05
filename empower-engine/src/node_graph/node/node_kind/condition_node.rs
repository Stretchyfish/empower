use crate::{node_graph::node::{NodeFunction, port::{PortCompatability, PortValue}}, utility::text_buffer::TextBuffer};

use super::NodeKind;

#[derive(Clone)]
pub struct ConditionNode
{
    
}

impl NodeKind for ConditionNode
{
    fn new() -> Box<dyn NodeKind> where
        Self: Sized {

        Box::new( Self {} )
    }

    fn name(&self) -> &'static str {
        "condition"
    }

    fn clone_box(&self) -> Box<dyn NodeKind> {
        Box::new( self.clone() )
    }

    fn function(&self) -> NodeFunction {
        NodeFunction::Instant
    }

    fn input_compatabilities(&self) -> Vec<PortCompatability> {
        vec![
            PortCompatability::Exatch( PortValue::Trigger(false) ),
            PortCompatability::OneOf( vec![ PortValue::Integer(0), PortValue::Float(0.0) ]),
            PortCompatability::OneOf( vec![ PortValue::Integer(0), PortValue::Float(0.0) ])
        ]
    }

    fn output_compatabilities(&self) -> Vec<PortCompatability> {
        vec![
            PortCompatability::Exatch( PortValue::Trigger(false) ),
            PortCompatability::Exatch( PortValue::Trigger(false) )
        ]
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn setup(&mut self, inputs: Vec<&PortValue>, log: &mut TextBuffer) -> Option<Vec<PortValue>> {

        let compare_value = match inputs[1]
        {
            PortValue::Integer(val) => val,
            _ => panic!("Imposible value!"),
        };

        let value = match inputs[2]
        {
            PortValue::Integer(val) => val,
            _ => panic!("Imposible value!"),
        };

        if value > compare_value
        {
            return Some( vec![ PortValue::Trigger(true), PortValue::Trigger(false) ] );
        }

        Some( vec![ PortValue::Trigger(false), PortValue::Trigger(true) ] )
    }

    fn update(&mut self) -> Option<Vec<PortValue>> {
        todo!()
    }

    fn execute(&mut self, ui: &mut egui::Ui) {
        todo!()
    }
}
