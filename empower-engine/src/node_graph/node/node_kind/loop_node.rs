use crate::{compiler::Instruction, node_graph::{NodeEdit, node::node_kind::{ControlFlowKind, NodeState}, port::PortDefinition}, value::Value};
use serde::{Deserialize, Serialize};

use super::NodeKind;
use super::LoopSettings;

#[derive(Clone, Serialize, Deserialize)]
pub struct LoopNode
{
    state: Vec<NodeEdit>,
}

#[typetag::serde]
impl NodeKind for LoopNode
{
    fn new() -> Box<dyn NodeKind> where
        Self: Sized {

        Box::new( Self {
            state: vec![
                NodeEdit::EnumBox { label: String::from("mode"), states: vec![String::from("forever"), String::from("range")], current_state: String::from("forever") },
                // @TODO, this whole state setup can be simplified
            ],
        } )
    }

    fn clone_box(&self) -> Box<dyn NodeKind> {

        Box::new( self.clone() )
    }

    fn name(&self) -> &'static str {
        "loop"
    }

    fn size(&self) -> egui::Vec2 {

        let selected_state = match &self.state[0]
        {
            NodeEdit::EnumBox { label: _, states: _, current_state: text } => text.as_str(),
            _ => todo!(),
        };

        match selected_state
        {
            "forever" => egui::vec2(230.0, 215.0),
            "range" => egui::vec2(320.0, 400.0),
            _ => todo!()
        }
    }

    fn input_port_definitions(&self) -> Vec<crate::node_graph::port::PortDefinition> {

        let selected_state = match &self.state[0]
        {
            NodeEdit::EnumBox { label: _, states: _, current_state: text } => text.as_str(),
            _ => todo!(),
        };

        match selected_state
        {
            "forever" =>
                    vec![
                        PortDefinition::new_input_execution_port(),
                    ],
            "range" =>
                    vec![
                        PortDefinition::new_input_execution_port(),
                        PortDefinition::new_input_data_port("start".to_string(), vec![ Value::Integer( 0 ) ] ),
                        PortDefinition::new_input_data_port("interval".to_string(), vec![ Value::Integer( 1 ) ] ),
                        PortDefinition::new_input_data_port("stop".to_string(), vec![ Value::Integer( 10 ) ] ),
                    ],
            _ => todo!(),
        }
    }

    fn output_port_definitions(&self) -> Vec<crate::node_graph::port::PortDefinition> {
        let selected_state = match &self.state[0]
        {
            NodeEdit::EnumBox { label: _, states: _, current_state: text } => text.as_str(),
            _ => todo!(),
        };

        match selected_state
        {
            "forever" =>
                    vec![
                        PortDefinition::new_output_execution_port(),
                    ],
            "range" =>
                    vec![
                        PortDefinition::new_output_execution_port(),
                        PortDefinition::new_output_data_port("condition".to_string(), vec![ Value::Bool( false ) ] ),
                        PortDefinition::new_output_data_port("value".to_string(), vec![ Value::Integer( 0 ) ] ),
                    ],
            _ => todo!(),
        }
    }

    fn node_edits(&mut self) -> Option<&mut Vec<super::NodeEdit>> {
        Some( &mut self.state )
    }

    fn sync_node_edit(&mut self, _: usize) -> super::NodeSyncResponse {
        super::NodeSyncResponse::NodesStructureChanged
    }

    fn sync_node_state(&mut self, _: NodeState) {
        
    }

    fn compile(&self, ctx: &mut crate::compiler::CompiledGraphContext, input_port_addresses: Vec<crate::compiler::RegisterAddress>, output_port_addresses: Vec<crate::compiler::RegisterAddress>) {

        let selected_state = match &self.state[0] // @TODO, find a way to make this selected state easier to get
        {
            NodeEdit::EnumBox { label: _, states: _, current_state: text } => text.as_str(),
            _ => todo!(),
        };

        match selected_state
        {
            "forever" =>
            {

            },
            "range" =>
            {
                ctx.add_instruction( Instruction::Copy(input_port_addresses[0], output_port_addresses[1]) );
                ctx.add_instruction( Instruction::Compare(input_port_addresses[2], output_port_addresses[1], output_port_addresses[0] ) );
                ctx.add_instruction_placeholder( Instruction::JumpIfTrue(0, output_port_addresses[0]));
                ctx.add_instruction( Instruction::Add(output_port_addresses[1], input_port_addresses[1], output_port_addresses[1]));
            },
            _ => todo!(),
        }
    }

    fn control_flow(&self) -> super::ControlFlowKind {

        let selected_state = match &self.state[0] // @TODO, find a way to make this selected state easier to get
        {
            NodeEdit::EnumBox { label: _, states: _, current_state: text } => text.as_str(),
            _ => todo!(),
        };

        match selected_state
        {
            "forever" =>
            {
                ControlFlowKind::Loop( LoopSettings::Forever )
            },
            "range" =>
            {
                ControlFlowKind::Loop( LoopSettings::Interval )
            },
            _ => todo!(),
        }
    }
}
