use empower_engine::PortValue;
use empower_engine::node_graph::node::NodeKind;
use super::DisplayNodeKind;
use super::super::DisplayPort;

#[derive(Clone)]
pub struct ConditionDisplayNode
{

}

impl DisplayNodeKind for ConditionDisplayNode
{
    fn new() -> Box<dyn DisplayNodeKind> where
        Self: Sized {
        
        Box::new( Self {} )
    }

    fn clone_box(&self) -> Box<dyn DisplayNodeKind> {
        Box::new( self.clone() )
    }

    fn node_size(&self, _: &Box<dyn NodeKind>) -> egui::Vec2 {
        egui::Vec2 { x: 350.0, y: 300.0 }
    }

    fn state_size(&self) -> egui::Vec2 {
        egui::Vec2 { x: 0.0, y: 0.0 }
    }

    fn display_input_ports(&self, input_port_values: Vec<&PortValue>) -> Vec<DisplayPort> {

        let mut display_inputs = Vec::new();

        display_inputs.reserve(input_port_values.len());
        for input_port_value in input_port_values
        {
            let display_port = DisplayPort::new(
                                                            "A".to_string(), 
                                                            egui::pos2(0.0, 0.0), 
                                                            &input_port_value
                                                        );
            display_inputs.push(display_port);
        }

        display_inputs
    }

    fn state_show(&mut self, _: &mut egui::Ui, _: &mut Box<dyn NodeKind>) -> bool {
        false
    }
}
