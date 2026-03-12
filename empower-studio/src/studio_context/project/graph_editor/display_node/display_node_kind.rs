use std::collections::HashMap;

use empower_engine::{PortValue, node_graph::node::node_kind::NodeKind};
use once_cell::sync::Lazy;

mod default_display_node;
use default_display_node::DefaultDisplayNode;

mod file_path_display_node;
use file_path_display_node::FilePathDisplayNode;

mod number_display_node;
use number_display_node::NumberDisplayNode;

mod vector_display_node;
use vector_display_node::VectorDisplayNode;

mod math_graph_display_node;
use math_graph_display_node::MathGraphDisplayNode;

mod show_image_display_node;
use show_image_display_node::ShowImageDisplayNode;

mod condition_display_node;
use condition_display_node::ConditionDisplayNode;

mod display_wait_node;
use display_wait_node::DisplayWaitNode;

use super::super::DisplayPort; // @TODO, improve this include

#[typetag::serde(tag="dislay_node_kind")]
pub trait DisplayNodeKind
{
    fn new() -> Box<dyn DisplayNodeKind> // This constructor is to allow for dyn
    where
        Self: Sized;
    fn clone_box(&self) -> Box<dyn DisplayNodeKind>; // This is needed to enable trait cloning
    fn node_size(&self, node_kind: &Box<dyn NodeKind>) -> egui::Vec2;
    fn display_input_ports(&self, input_port_values: Vec<&PortValue>) -> Vec<DisplayPort>;
    fn display_output_ports(&self, output_port_values: Vec<&PortValue>) -> Vec<DisplayPort>;
    fn state_size(&self) -> egui::Vec2;
    fn state_show(&mut self, ui: &mut egui::Ui, node_kind: &mut Box<dyn NodeKind>) -> DisplayNodeStateResponse;
}

impl Clone for Box<dyn DisplayNodeKind>
{
    fn clone(&self) -> Self
    {
        self.clone_box()
    }
}

pub enum DisplayNodeStateResponse
{
    NoChange,
    RefreshNodeStructure,
}

type DisplayNodeConstructor = fn() -> Box<dyn DisplayNodeKind>;

pub static DISPLAY_NODE_KIND_REGISTRY: Lazy<HashMap<&'static str, DisplayNodeConstructor>> = Lazy::new(|| {
    let mut m: HashMap<&'static str, DisplayNodeConstructor> = HashMap::new();

    // This one cannot be removed, or it will cause a crash in display node generation
    m.insert("default", || DefaultDisplayNode::new() ); 
    
    m.insert("file path", || FilePathDisplayNode::new() ); 
    m.insert("number", || NumberDisplayNode::new() ); 
    m.insert("vector", || VectorDisplayNode::new() ); 
    m.insert("math graph", || MathGraphDisplayNode::new() ); 
    m.insert("show image", || ShowImageDisplayNode::new() ); 
    m.insert("condition", || ConditionDisplayNode::new() ); 
    m.insert("wait", || DisplayWaitNode::new() ); 

    m
});

