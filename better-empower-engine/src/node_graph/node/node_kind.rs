use std::collections::HashMap;
use std::any::Any;

use once_cell::sync::Lazy;

use super::port::{PortValue, PortCompatability};
use super::NodeFunction;

mod start_node;
use start_node::StartNode;

mod number_node;
use number_node::NumberNode;

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
use vector_node::VectorNode;

mod math_graph_node;
use math_graph_node::MathGraphNode;

mod file_path_node;
use file_path_node::FilePathNode;

mod show_image_node;
use show_image_node::ShowImageNode;

pub trait NodeKind
{
    fn new() -> Box<dyn NodeKind> // This constructor is to allow for dyn
    where
        Self: Sized;
    fn name(&self) -> &'static str;
    fn clone_box(&self) -> Box<dyn NodeKind>; // This is needed to enable trait cloning
    fn function(&self) -> NodeFunction; // @TODO, maybe rename it behavior?
    fn input_compatabilities(&self) -> Vec<PortCompatability>;
    fn output_compatabilities(&self) -> Vec<PortCompatability>;
    fn as_any(&self) -> &dyn Any; 
    fn state(&mut self, ui: &mut egui::Ui);
    fn setup(&mut self, inputs: Vec<&PortValue>) -> Option<Vec<PortValue>>;
    fn execute(&mut self, ui: Option<&mut egui::Ui>) -> Option<Vec<PortValue>>;
}

impl Clone for Box<dyn NodeKind>
{
    fn clone(&self) -> Self
    {
        self.clone_box()
    }
}

type NodeConstructor = fn() -> Box<dyn NodeKind>;

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

    m
});
