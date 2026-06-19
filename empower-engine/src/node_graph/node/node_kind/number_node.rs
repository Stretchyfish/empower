use crate::compiler::CompiledGraphContext;
use crate::compiler::Instruction;
use crate::compiler::RegisterAddress;
use crate::node_graph::node::node_kind::NodeSyncResponse;
use crate::node_graph::port::PortDefinition;
use crate::value::Value;

use super::NodeKind;
use super::ControlFlowKind;
use super::NodeEdit;

#[derive(Clone)]
pub struct NumberNode
{
    state: Vec<NodeEdit>,
}

impl NodeKind for NumberNode
{
    fn new() -> Box<dyn NodeKind> where
        Self: Sized {

        Box::new(
            Self {
                state: vec![
                            NodeEdit::Text { label: "value", text: "0".to_string(), parseble: true }
                ],
            }
        )
    }

    fn clone_box(&self) -> Box<dyn NodeKind> {
        Box::new( self.clone() )
    }

    fn name(&self) -> &'static str {
        "number"
    }

    fn size(&self) -> egui::Vec2 {
        egui::vec2(230.0, 215.0)
    }

    fn input_port_definitions(&self) -> Vec<PortDefinition> {
        vec![
            PortDefinition::new_input_data_port("value".to_string(), vec![ Value::Integer(0) ]),
        ]
    }

    fn output_port_definitions(&self) -> Vec<PortDefinition> {
        vec![
            PortDefinition::new_output_data_port("value".to_string(), vec![ Value::Integer(0) ]),
        ]
    }

    fn node_edits(&mut self) -> Option<&mut Vec<NodeEdit>> {
        Some( &mut self.state )
    }

    fn sync_node_edit(&mut self, _: usize) -> NodeSyncResponse {
        todo!()
    }

    fn compile(&self, ctx: &mut CompiledGraphContext, input_registers_addresses: Vec<RegisterAddress>, output_register_addresses: Vec<RegisterAddress>) {
        ctx.add_instruction(
            Instruction::Copy(input_registers_addresses[0], output_register_addresses[0]),
            // Instruction::SetConst(output_register_addresses[0], Value::Integer( state ))
        );
    }

    fn control_flow(&self) -> ControlFlowKind {
        ControlFlowKind::Normal
    }
}
