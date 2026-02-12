use std::collections::HashMap;
use std::any::Any; 

use once_cell::sync::Lazy;

use super::port::{PortValue, PortCompatability};

mod start_node;
use start_node::StartNode;

mod number_node;
pub use number_node::NumberNode;
pub use number_node::NumberNodeValueKind;

mod addition_node;
use addition_node::AdditionNode;

mod multiply_node;
use multiply_node::MultiplyNode;

mod boolean_node;
use boolean_node::BooleanNode;

mod print_node;
use print_node::PrintNode;

mod text_node;
use text_node::TextNode;

mod vector_node;
pub use vector_node::VectorNode;

mod math_graph_node;
use math_graph_node::MathGraphNode;

mod file_path_node;
pub use file_path_node::FilePathNode;

mod show_image_node;
use show_image_node::ShowImageNode;

mod condition_node;
pub use condition_node::ConditionNode;
pub use condition_node::ConditionType;

mod loop_node;
pub use loop_node::LoopNode;

mod wait_node;
pub use wait_node::WaitNode;
pub use wait_node::WaitTimeIntervals;

mod restart_loop_node;
use restart_loop_node::RestartLoopNode;

mod stop_loop_node;
use stop_loop_node::StopLoopNode;

#[typetag::serde(tag="node_kind")]
pub trait NodeKind
{
    fn new() -> Box<dyn NodeKind> // This constructor is to allow for dyn
    where
        Self: Sized;
    fn name(&self) -> &'static str;
    fn clone_box(&self) -> Box<dyn NodeKind>; // This is needed to enable trait cloning
    fn input_compatabilities(&self) -> Vec<PortCompatability>;
    fn output_compatabilities(&self) -> Vec<PortCompatability>;
    fn as_any_mut(&mut self) -> &mut dyn Any; 
    fn as_any(&self) -> &dyn Any; 
    fn setup(&mut self, inputs: Vec<&PortValue>) -> NodeSetupResponse;
    fn update(&mut self) -> NodeUpdateResponse;
    fn show(&mut self, ui: &mut egui::Ui); // @TODO, consider renaming show or other?
}

impl Clone for Box<dyn NodeKind>
{
    fn clone(&self) -> Self
    {
        self.clone_box()
    }
}

pub enum NodeSetupResponse
{
    Began,
    Finished(Vec<PortValue>),
    FinishedWithLog(Vec<PortValue>, String),
    CreateWindow,
    CreateLoop(Vec<PortValue>),
    RestartLoop,
    StopLoop,
    Error(String),
}

pub enum NodeUpdateResponse
{
    Finished(Vec<PortValue>),
    Running, // @TODO, find a better name
    ContinueLoop(Vec<PortValue>),
}

type NodeConstructor = fn() -> Box<dyn NodeKind>;

// @TODO, rename this to node_kind_registry
pub static NODE_REGISTRY: Lazy<HashMap<&'static str, NodeConstructor>> = Lazy::new(|| {
    let mut m: HashMap<&'static str, fn() -> Box<dyn NodeKind>> = HashMap::new();

    m.insert(StartNode::new().name(), || StartNode::new());
    m.insert(NumberNode::new().name(), || NumberNode::new());
    m.insert(AdditionNode::new().name(), || AdditionNode::new());
    m.insert(MultiplyNode::new().name(), || MultiplyNode::new());
    m.insert(BooleanNode::new().name(), || BooleanNode::new());
    m.insert(PrintNode::new().name(), || PrintNode::new());
    m.insert(TextNode::new().name(), || TextNode::new());
    m.insert(VectorNode::new().name(), || VectorNode::new());
    m.insert(MathGraphNode::new().name(), || MathGraphNode::new());
    m.insert(FilePathNode::new().name(), || FilePathNode::new());
    m.insert(ShowImageNode::new().name(), || ShowImageNode::new());
    m.insert(ConditionNode::new().name(), || ConditionNode::new());
    m.insert(LoopNode::new().name(), || LoopNode::new());
    m.insert(WaitNode::new().name(), || WaitNode::new());
    m.insert(RestartLoopNode::new().name(), || RestartLoopNode::new());
    m.insert(StopLoopNode::new().name(), || StopLoopNode::new());

    m
});
