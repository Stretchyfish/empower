use crate::compiler::Instruction;
use crate::value::Value;
use crate::node_graph::port::PortDefinition;

use super::NodeKind;
use super::ControlFlowKind;

#[derive(Clone)]
pub struct PrintNode
{
    
}

impl NodeKind for PrintNode
{
    fn new() -> Box<dyn NodeKind> where
        Self: Sized {

        Box::new( Self {} )
    }

    fn clone_box(&self) -> Box<dyn NodeKind> {
        Box::new( self.clone() )
    }

    fn name(&self) -> &'static str {
        "print"
    }

    fn size(&self) -> egui::Vec2 {
        egui::vec2(300.0, 220.0)
    }

    fn input_port_definitions(&self) -> Vec<PortDefinition>
    {
        vec![
            PortDefinition::new_input_execution_port(),
            PortDefinition::new_input_data_port("value", vec![ Value::Integer, Value::Float])
        ]
    }

    fn output_port_definitions(&self) -> Vec<PortDefinition>
    {
        Vec::new()
    }

    fn compile(&self) -> Vec<Instruction> {
        vec![
            Instruction::Print("Hello World".to_string())
        ]
    }

    fn control_flow(&self) -> ControlFlowKind {
        ControlFlowKind::Normal
    }

    fn node_edits(&mut self) -> Option<&mut Vec<super::NodeEdit>> {
        None
    }

    fn sync_node_edit(&mut self, index: usize) {
        todo!()
    }
}
