use std::collections::HashMap;
use std::ops::Mul;

use crate::node_graph::node::NodeKind2;

use once_cell::sync::Lazy;

type NodeConstructor = fn() -> Box<dyn NodeKind2>;

use super::node_kind::start_node::StartNode;
use super::node_kind::text_node::TextNode;
use super::node_kind::bool_node::BoolNode;
use super::node_kind::number_node::NumberNode;
use super::node_kind::addition_node::AdditionNode;
use super::node_kind::multiply_node::MultiplyNode;
use super::node_kind::print_node::PrintNode;


pub static NODE_REGISTRY: Lazy<HashMap<&'static str, NodeConstructor>> = Lazy::new(|| {
    let mut m: HashMap<&'static str, fn() -> Box<dyn NodeKind2>> = HashMap::new();
    m.insert("start", || Box::new( StartNode {} ));
    m.insert("number", || Box::new( NumberNode {} ));
    m.insert("text", || Box::new( TextNode {} ));
    m.insert("addition", || Box::new( AdditionNode {} ));
    m.insert("multiply", || Box::new( MultiplyNode {} ));
    m.insert("bool", || Box::new( BoolNode {} ));
    m.insert("print", || Box::new( PrintNode {} ));

    m 
});

pub struct NodeRegistry
{

}

impl NodeRegistry
{
    pub fn new() -> Self
    {
        NodeRegistry {  }
    }

}