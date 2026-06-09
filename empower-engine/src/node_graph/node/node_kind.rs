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

    fn input_port_definitions(&self) -> Vec<PortDefinition>;
    fn output_port_definitions(&self) -> Vec<PortDefinition>;

    fn draw_state(&mut self, ui: &mut egui::Ui) -> DrawnStateResponse;

    fn compile(&self) -> Vec<Instruction>;

    fn control_flow(&self) -> ControlFlowKind;
}

pub enum DrawnStateResponse
{
    None,
    UpdatedNodeStructure
}

pub enum ControlFlowKind
{
    Normal,
    If,
    Loop,
    Break,
    Return
}

type NodeConstructor = fn() -> Box<dyn NodeKind>;

pub static NODE_KIND_REGISTRY: Lazy<HashMap<&'static str, NodeConstructor>> = Lazy::new(|| {
    let mut r: HashMap<&'static str, NodeConstructor> = HashMap::new();

    r.insert( PrintNode::new().name(), || PrintNode::new());
    r.insert( StartNode::new().name(), || StartNode::new());
    r.insert( NumberNode::new().name(), || NumberNode::new());

    r
});
