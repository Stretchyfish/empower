use crate::{node_graph::{NodeEdit, node::node_kind::NodeSyncResponse, port::PortDefinition}, utility::alphabet_counter::AlphabetCounter, value::Value};

use super::NodeKind;

#[derive(Clone)]
pub struct ListNode
{
    state: Vec<NodeEdit>,
    size: usize,
}

impl NodeKind for ListNode
{
    fn new() -> Box<dyn NodeKind> where
        Self: Sized {

        Box::new( Self {

            state: vec![
                NodeEdit::Text { label: "size", text: "2".to_string(), parseble: true }
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
        egui::vec2(230.0, 215.0)
    }

    fn input_port_definitions(&self) -> Vec<crate::node_graph::port::PortDefinition> {

        let mut inputs = Vec::with_capacity(self.size);

        let mut alphabet_counter = AlphabetCounter::new();

        for _ in 0..self.size
        {
            let letter = alphabet_counter.next_letter().to_string();
            inputs.push( PortDefinition::new_input_data_port(letter, vec![ Value::Integer(0) ]) );
        }
        
        inputs
    }

    fn output_port_definitions(&self) -> Vec<crate::node_graph::port::PortDefinition> {
        Vec::new()
    }

    fn node_edits(&mut self) -> Option<&mut Vec<super::NodeEdit>> {
        Some( &mut self.state )
    }

    fn sync_node_edit(&mut self, index: usize) -> NodeSyncResponse {

        let node_edit = &mut self.state[index];

        let size = node_edit.parse_to_usize();

        if size.is_err()
        {
            return NodeSyncResponse::Nothing;
        }

        self.size = size.unwrap();

        NodeSyncResponse::NodesStructureChanged
    }

    fn compile(&self, ctx: &mut crate::compiler::CompiledGraphContext, input_port_register_adresses: Vec<crate::compiler::RegisterAddress>, output_port_register_adresses: Vec<crate::compiler::RegisterAddress>) {
        todo!()
    }

    fn control_flow(&self) -> super::ControlFlowKind {
        super::ControlFlowKind::Normal
    }
}
