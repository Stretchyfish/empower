use crate::{node_graph::node::{NodeFunction, port::{PortCompatability, PortValue}}, utility::text_buffer::TextBuffer};

use super::NodeKind;

#[derive(Clone)]
pub struct VectorNode
{
    pub number_of_input_ports: i32,
}

impl NodeKind for VectorNode
{
    fn new() -> Box<dyn NodeKind> where
        Self: Sized {
        
        Box::new( Self { number_of_input_ports: 2 } )
    }

    fn name(&self) -> &'static str {
        "vector"
    }

    fn clone_box(&self) -> Box<dyn NodeKind> {
        Box::new( self.clone() )
    }

    fn function(&self) -> NodeFunction {
        NodeFunction::Instant
    }

    fn input_compatabilities(&self) -> Vec<PortCompatability> {

        if self.number_of_input_ports < 2
        {
            panic!("Vector somehow got an impossible size");
        }

        vec![ PortCompatability::OneOf( vec![PortValue::Integer(0), PortValue::Float(0.0), PortValue::Text( String::new() ), PortValue::Bool( false ) ]); self.number_of_input_ports as usize]
    }

    fn output_compatabilities(&self) -> Vec<PortCompatability> {
        Vec::from(
            [
                PortCompatability::Exatch( PortValue::Vector( Vec::new() )),
            ]
        )
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
       self 
    }

    fn setup(&mut self, inputs: Vec<&PortValue>, _: &mut TextBuffer) -> Option<Vec<PortValue>> {
        let mut port_values_vector = Vec::new();
        port_values_vector.reserve(inputs.len());

        for port_value in inputs
        {
            port_values_vector.push(port_value.clone());
        }

        Some( vec![ PortValue::Vector( port_values_vector ) ] )
 
    }

    fn update(&mut self) -> Option<Vec<PortValue>> {
        todo!()
    }

    fn execute(&mut self, _: &mut egui::Ui) {
        todo!()
    }
    
}