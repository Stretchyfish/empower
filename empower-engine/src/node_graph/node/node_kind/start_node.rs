use crate::node_graph::port::{PortCompatability, PortValue};

use crate::analyser::TextBuffer;

use super::NodeKind2;

#[derive(Default, Clone)]
pub struct StartNode
{

}

impl NodeKind2 for StartNode
{
    fn name(&self) -> &'static str 
    {
        "start"
    }

    fn input_ports_compatabilities(&self) -> Vec<PortCompatability> 
    {
        Vec::new()
    }

    fn output_ports_compatabilities(&self) -> Vec<PortCompatability> 
    {
        Vec::from(
                [ 
                        PortCompatability::Exatch( PortValue::Trigger )
                ]
            )
    }

    fn execute(&self, inputs: Vec<&PortValue>, log: &mut TextBuffer) -> Vec<PortValue> 
    {
        println!("Ran start node");
        Vec::from( [ PortValue::Trigger ] )
    }
}

pub fn get_start_node_input_ports_compatabilities() -> Vec<PortCompatability>
{
    Vec::new()
}

pub fn get_start_node_output_ports_compatabilities() -> Vec<PortCompatability>
{
   Vec::from(
        [ 
                PortCompatability::Exatch( PortValue::Trigger )
        ]
    )
}

pub fn execute_start_node() -> Vec<PortValue>
{
    println!("Ran start node");
    Vec::from( [ PortValue::Trigger ] )
}