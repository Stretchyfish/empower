use crate::assets::Assets;
use crate::compiler::CompiledGraphContext;
use crate::compiler::Instruction;
use crate::compiler::RegisterAddress;
use crate::node_graph::NodeEdit;
use crate::node_graph::Port;
use crate::node_graph::node::node_kind::NodeState;
use crate::node_graph::port::PortDefinition;
use crate::node_graph::port::PortDirection;
use crate::value::Value;
use super::ControlFlowKind;
use super::NodeSyncResponse; 
use super::NodeKind;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct SubGraphNode
{
    edit_state: Vec<NodeEdit>,

    graph_start_node_input_port_definitions: Vec<PortDefinition>,
    graph_end_node_output_port_definitions: Vec<PortDefinition>,
}

#[typetag::serde]
impl NodeKind for SubGraphNode
{
    fn new() -> Box<dyn NodeKind>where Self:Sized {

        Box::new( Self {

            edit_state: vec![
                NodeEdit::GraphSelector { graph_id: None }, 
            ],

            graph_start_node_input_port_definitions: Vec::new(),
            graph_end_node_output_port_definitions: Vec::new(),
        })
    }

    fn clone_box(&self) -> Box<dyn NodeKind>  {

        Box::new( self.clone() )
    }

    fn name(&self) ->  &'static str {
        "sub graph"
    }

    fn size(&self) -> egui::Vec2 {
        egui::vec2(300.0, 220.0)
    }

    fn input_port_definitions(&self) -> Vec<PortDefinition>  {
        self.graph_start_node_input_port_definitions.clone()
    }

    fn output_port_definitions(&self) -> Vec<PortDefinition>  {
        Vec::new()
    }

    fn node_edits(&mut self) -> Option< &mut Vec<NodeEdit> >  {
        Some( &mut self.edit_state )
    }

    fn sync_node_edit(&mut self, _: usize) -> NodeSyncResponse {

        let graph_id = match self.edit_state[0] // @TODO, this is not a great approach, and should be fixed in the future
        {
            NodeEdit::GraphSelector { graph_id } => graph_id.unwrap(),
            _ => { return NodeSyncResponse::Nothing; }, 
        };

        println!("Sub graph id to edit: {}", graph_id);

        NodeSyncResponse::LoadSubgraph( graph_id )
    }

    fn sync_node_state(&mut self, node_state: NodeState) {

        let (input_ports, _) = match node_state
        {
            NodeState::GraphStartAndEndPorts { start_input_ports, end_output_ports } => (start_input_ports, end_output_ports),
        };

        self.graph_start_node_input_port_definitions = Vec::with_capacity( input_ports.len() );

        for port in input_ports
        {
            let mut port_definition = port.to_port_definitions();
            port_definition.direction = PortDirection::Input;

            self.graph_start_node_input_port_definitions.push( port_definition );
        }

        // for port in output_ports
        // {
        //     self.graph_end_node_output_port_definitions.push( port.to_port_definitions() );
        // }
    }

    fn compile(&self, ctx: &mut CompiledGraphContext, input_port_register_adresses: Vec<RegisterAddress>, output_port_register_adresses: Vec<RegisterAddress>) {
    }

    fn control_flow(&self) -> ControlFlowKind {
        ControlFlowKind::Normal
    }
}

impl SubGraphNode
{
    // pub fn new(assets: &Assets) -> Self
    // {
    //     let graph = assets.get_node_graph()

    //     Self
    //     {
            
    //     }

        
    // }
}
