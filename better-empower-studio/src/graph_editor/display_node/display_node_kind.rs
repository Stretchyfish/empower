use std::collections::HashMap;
use std::any::Any; // @TODO, think I can remove this now and its affect on the traits

use better_empower_engine::node_graph::node::NodeKind;
use once_cell::sync::Lazy;

mod default_display_node;
use default_display_node::DefaultDisplayNode;

use super::super::DisplayPort; // @TODO, improve this include

pub trait DisplayNodeKind
{
    fn new() -> Box<dyn DisplayNodeKind> // This constructor is to allow for dyn
    where
        Self: Sized;
    fn clone_box(&self) -> Box<dyn DisplayNodeKind>; // This is needed to enable trait cloning
    fn node_size(&self) -> egui::Vec2;
    fn display_inputs(&self, node_kind: &Box<dyn NodeKind>) -> Vec<DisplayPort>;
    fn state_size(&self) -> egui::Vec2;
    fn state_show(&mut self, ui: &mut Option<egui::Ui>) -> bool; // The bool indicates a change 
}

impl Clone for Box<dyn DisplayNodeKind>
{
    fn clone(&self) -> Self
    {
        self.clone_box()
    }
}

type DisplayNodeConstructor = fn() -> Box<dyn DisplayNodeKind>;

pub static DISPLAY_NODE_KIND_REGISTRY: Lazy<HashMap<&'static str, DisplayNodeConstructor>> = Lazy::new(|| {
    let mut m: HashMap<&'static str, DisplayNodeConstructor> = HashMap::new();

    // This one cannot be removed, or it will cause a crash in display node generation
    m.insert("default", || DefaultDisplayNode::new() ); 

    m
});

