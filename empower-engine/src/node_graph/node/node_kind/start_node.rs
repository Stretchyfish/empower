use crate::node_graph::port::{PortCompatability, PortValue};

use crate::analyser::TextBuffer;

use super::NodeKind;

#[derive(Default, Clone)]
pub struct StartNode
{

}

impl NodeKind for StartNode
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

    fn execute(&self, _: Vec<&PortValue>, _: &mut TextBuffer) -> Vec<PortValue> 
    {
        println!("Ran start node");
        Vec::from( [ PortValue::Trigger ] )
    }
}
