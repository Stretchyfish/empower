use std::collections::HashMap;
use once_cell::sync::Lazy;

use crate::{compiler::Instruction, node_graph::port::PortDefinition};

mod print_node;
use print_node::PrintNode;

mod start_node;
use start_node::StartNode;

mod number_node;
use number_node::NumberNode;

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

    fn sync_node_edit(&mut self, index: usize);

    fn compile(&self) -> Vec<Instruction>;

    fn control_flow(&self) -> ControlFlowKind;
}

pub enum ControlFlowKind
{
    Normal,
    If,
    Loop,
    Break,
    Return
}

#[derive(Clone)]
pub enum NodeEdit
{
    Text { label: &'static str, text: String, parseble: bool },
}

impl NodeEdit
{
    pub fn ui(&mut self, ui: &mut egui::Ui) -> egui::Response
    {
        match self
        {
            NodeEdit::Text { label, text, parseble } => ui.text_edit_singleline(text) ,
        }
    }
}

type NodeConstructor = fn() -> Box<dyn NodeKind>;

pub static NODE_KIND_REGISTRY: Lazy<HashMap<&'static str, NodeConstructor>> = Lazy::new(|| {
    let mut r: HashMap<&'static str, NodeConstructor> = HashMap::new();

    r.insert( PrintNode::new().name(), || PrintNode::new());
    r.insert( StartNode::new().name(), || StartNode::new());
    r.insert( NumberNode::new().name(), || NumberNode::new());

    r
});
