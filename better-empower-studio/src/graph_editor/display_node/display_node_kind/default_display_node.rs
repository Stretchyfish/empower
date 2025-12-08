use better_empower_engine::node_graph::node::NodeKind;

use super::DisplayNodeKind;

use super::DisplayPort;

#[derive(Clone)]
pub struct DefaultDisplayNode
{

}

impl DisplayNodeKind for DefaultDisplayNode
{
    fn new() -> Box<dyn DisplayNodeKind> where
        Self: Sized {
        
        Box::new( Self {} )
    }

    fn clone_box(&self) -> Box<dyn DisplayNodeKind> {
        Box::new( self.clone() )
    }

    fn node_size(&self) -> egui::Vec2 {
        egui::Vec2 { x: 350.0, y: 230.0 }
    }

    fn state_size(&self) -> egui::Vec2 {
        egui::Vec2 { x: 0.0, y: 0.0 }
    }

    fn display_inputs(&self, node_kind: &Box<dyn NodeKind>) -> Vec<DisplayPort> {

        let input_compatabilities = node_kind.input_compatabilities();

        let mut display_inputs = Vec::new();
        display_inputs.reserve(input_compatabilities.len());
        for compatability in input_compatabilities
        {
            let display_port = DisplayPort::new("A", egui::pos2(0.0, 0.0));
            display_inputs.push(display_port);
        }

        display_inputs
    }

    fn state_show(&mut self, _: &mut Option<egui::Ui>) -> bool {
        false
    }
}