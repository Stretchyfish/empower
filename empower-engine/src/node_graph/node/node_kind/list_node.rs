use crate::{compiler::{CompiledGraphContext, Instruction, RegisterAddress}, node_graph::{NodeEdit, node::node_kind::{NodeState, NodeSyncResponse}, port::PortDefinition}, utility::alphabet_counter::AlphabetCounter, value::Value};

use serde::{Deserialize, Serialize};
use super::NodeKind;

#[derive(Clone, Serialize, Deserialize)]
pub struct ListNode
{
    state: Vec<NodeEdit>,
    size: usize,
}

#[typetag::serde]
impl NodeKind for ListNode
{
    fn new() -> Box<dyn NodeKind> where
        Self: Sized {

        Box::new( Self {

            state: vec![
                NodeEdit::Text { label: "size".to_string(), text: "2".to_string(), parseble: true },
                NodeEdit::EnumBox { label: String::from("value"), states: vec![String::from("int"), String::from("float"), String::from("point2d")], current_state: String::from("float") },
            ],
            size: 2,
        })
    }

    fn clone_box(&self) -> Box<dyn NodeKind> {

        Box::new( self.clone() )
    }

    fn name(&self) -> &'static str {
        "list"
    }

    fn size(&self) -> egui::Vec2 {
        egui::vec2(230.0, 215.0 + 60.0 * self.size as f32 )
    }

    fn input_port_definitions(&self) -> Vec<crate::node_graph::port::PortDefinition> {

        let mut inputs = Vec::with_capacity(self.size);

        let mut alphabet_counter = AlphabetCounter::new();

        let selected_state = match &self.state[1]
        {
            NodeEdit::EnumBox { label: _, states: _, current_state: text } => text.as_str(),
            _ => todo!(),
        };

        let base_value = match selected_state
        {
            "int" => Value::Integer( 0 ),
            "float" => Value::Float( 0.0 ),
            "point2d" => Value::Point2d( 0.0, 0.0 ),
            _ => todo!()
        };

        for _ in 0..self.size
        {
            let letter = alphabet_counter.next_letter().to_string();
            inputs.push( PortDefinition::new_input_data_port(letter, vec![ base_value.clone() ]) );
        }
        
        inputs
    }

    fn output_port_definitions(&self) -> Vec<crate::node_graph::port::PortDefinition> {
        vec![
            PortDefinition::new_output_data_port("".to_string(), vec![ Value::List( Vec::new() ) ])
        ]
    }

    fn node_edits(&mut self) -> Option<&mut Vec<super::NodeEdit>> {
        Some( &mut self.state )
    }

    fn sync_node_edit(&mut self, index: usize) -> NodeSyncResponse {

        match index
        {
            0 =>
            {
                let node_edit = &mut self.state[index];

                let size = node_edit.parse_to_usize();

                if size.is_err()
                {
                    return NodeSyncResponse::Nothing;
                }

                self.size = size.unwrap();

                NodeSyncResponse::NodesStructureChanged
            }
            1 =>
            {
                NodeSyncResponse::NodesStructureChanged
            }
            _ =>
            {
                todo!()
            }
        }
    }

    fn sync_node_state(&mut self, _: NodeState) {
        
    }

    fn compile(&self, ctx: &mut CompiledGraphContext, input_registers_addresses: Vec<RegisterAddress>, output_register_addresses: Vec<RegisterAddress>) {
        ctx.add_instruction(
            Instruction::CreateList( input_registers_addresses.clone(), output_register_addresses[0]),
        );
    }

    fn control_flow(&self) -> super::ControlFlowKind {
        super::ControlFlowKind::None
    }
}
