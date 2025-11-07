use std::collections::HashMap;
use std::any::Any;

use once_cell::sync::Lazy;

use super::port::{PortValue, PortCompatability};
use super::NodeFunction;

mod start_node;
use start_node::StartNode;

mod number_node;
use number_node::NumberNode;

pub trait NodeKind
{
    fn new() -> Box<dyn NodeKind> // This constructor is to allow for dyn
    where
        Self: Sized;
    fn name(&self) -> &'static str;
    fn function(&self) -> NodeFunction;
    fn input_compatabilities(&self) -> Vec<PortCompatability>;
    fn output_compatabilities(&self) -> Vec<PortCompatability>;
    fn as_any(&self) -> &dyn Any; 
    fn state(&mut self, ui: &mut egui::Ui);
    fn setup(&mut self, inputs: Vec<&PortValue>);
    fn update(&mut self, ui: &mut egui::Ui) -> Option<Vec<PortValue>>;
}

type NodeConstructor = fn() -> Box<dyn NodeKind>;

pub static NODE_REGISTRY: Lazy<HashMap<&'static str, NodeConstructor>> = Lazy::new(|| {
    let mut m: HashMap<&'static str, fn() -> Box<dyn NodeKind>> = HashMap::new();

    m.insert(StartNode::new().name(), || StartNode::new());
    m.insert(NumberNode::new().name(), || NumberNode::new());

    m
});
