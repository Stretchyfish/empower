use crate::node_graph::port::{PortCompatability, PortValue};
use super::NodeKindTrait;

pub fn get_name() -> &'static str
{
    "start"
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

pub fn execute_start_node() -> Option<Vec<PortValue>>
{
    println!("Ran start node");
    Some( Vec::from( [ PortValue::Trigger ] ) )
}

pub struct StartNode
{

}

impl NodeKindTrait for StartNode
{
    fn new() -> Self {
        todo!()
    }

    fn name() -> &'static str {
        todo!()
    }

    fn input_compatabilities(self) -> Vec<PortCompatability> {
        todo!()
    }

    fn output_compatabilities(self) -> Vec<PortCompatability> {
        todo!()
    }

    fn state(&mut self, ui: &mut egui::Ui) {
        todo!()
    }

    fn setup(&mut self, inputs: Vec<&PortValue>) {
        todo!()
    }

    fn update(&mut self, ctx: &egui::Context) -> Option<Vec<PortValue>> {
        todo!()
    }
}
