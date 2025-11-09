use std::collections::HashMap;
use std::any::Any;
use std::ops::Mul;

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

    m
});
