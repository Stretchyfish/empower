use std::{collections::HashMap, num::{ParseFloatError, ParseIntError}};
use once_cell::sync::Lazy;

use crate::{compiler::{CompiledGraphContext, RegisterAddress}, node_graph::port::PortDefinition};

mod print_node;
use print_node::PrintNode;

mod start_node;
use start_node::StartNode;

mod number_node;
use number_node::NumberNode;

mod list_node;
use list_node::ListNode;

mod branch_node;
use branch_node::BranchNode;

mod wait_node;
use wait_node::WaitNode;

pub trait NodeKind
{
    fn new() -> Box<dyn NodeKind> // This constructor is to allow for dyn
    where
        Self: Sized;

    fn clone_box(&self) -> Box<dyn NodeKind>;
    fn name(&self) -> &'static str; 
    fn size(&self) -> egui::Vec2;

    fn input_port_definitions(&self) -> Vec<PortDefinition>;
    fn output_port_definitions(&self) -> Vec<PortDefinition>;

    fn node_edits(&mut self) -> Option<&mut Vec<NodeEdit>>;

    fn sync_node_edit(&mut self, index: usize) -> NodeSyncResponse;

    fn compile(&self, ctx: &mut CompiledGraphContext, input_port_register_adresses: Vec<RegisterAddress>, output_port_register_adresses: Vec<RegisterAddress>);

    fn control_flow(&self) -> ControlFlowKind;
}

pub enum ControlFlowKind
{
    Normal,
    Branch,
    Loop,
}

#[derive(Clone)]
pub enum NodeEdit
{
    Text { label: &'static str, text: String, parseble: bool },
    CheckBox { toggle: bool },
}

impl NodeEdit
{
    pub fn parse_to_usize(&mut self) -> Result<usize, ParseIntError>
    {
        let (text, parse) = match self
        {
            NodeEdit::Text { label: _, text, parseble } => ( text, parseble ),
            _ => panic!("list node has an invalid node edit")
        };

        let parsed_value = text.parse::<usize>();

        *parse = parsed_value.is_ok();

        parsed_value
    }

    pub fn parse_to_i32(&mut self) -> Result<i32, ParseIntError>
    {
        let (text, parse) = match self
        {
            NodeEdit::Text { label: _, text, parseble } => ( text, parseble ),
            _ => panic!("list node has an invalid node edit")
        };

        let parsed_value = text.parse::<i32>();

        *parse = parsed_value.is_ok();

        parsed_value
    }

    pub fn parse_to_f32(&mut self) -> Result<f32, ParseFloatError>
    {
        let (text, parse) = match self
        {
            NodeEdit::Text { label: _, text, parseble } => ( text, parseble ),
            _ => panic!("list node has an invalid node edit")
        };

        let parsed_value = text.parse::<f32>();

        *parse = parsed_value.is_ok();

        parsed_value
    }
}

pub enum NodeSyncResponse
{
    Nothing,
    NodesStructureChanged,
}

type NodeConstructor = fn() -> Box<dyn NodeKind>;

pub static NODE_KIND_REGISTRY: Lazy<HashMap<&'static str, NodeConstructor>> = Lazy::new(|| {
    let mut r: HashMap<&'static str, NodeConstructor> = HashMap::new();

    r.insert( PrintNode::new().name(), || PrintNode::new());
    r.insert( StartNode::new().name(), || StartNode::new());
    r.insert( NumberNode::new().name(), || NumberNode::new());
    r.insert( ListNode::new().name(), || ListNode::new());
    r.insert( BranchNode::new().name(), || BranchNode::new());
    r.insert( WaitNode::new().name(), || WaitNode::new());

    r
});
