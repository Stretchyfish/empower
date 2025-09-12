use std::collections::HashMap;
use once_cell::sync::Lazy;

use super::display_node_kind::DisplayNodeKind; 

use super::display_node_kind::display_addition_node::DisplayAdditionNode;
use super::display_node_kind::display_start_node::DisplayStartNode;
use super::display_node_kind::display_number_node::DisplayNumberNode;
use super::display_node_kind::display_bool_node::DisplayBoolNode;
use super::display_node_kind::display_print_node::DisplayPrintNode;
use super::display_node_kind::display_text_node::DisplayTextNode;
use super::display_node_kind::display_multiplication_node::DisplayMultiplyNode;


type DisplayNodeConstructor = fn() -> Box<dyn DisplayNodeKind>;

pub static DISPLAY_NODE_REGISTRY: Lazy<HashMap<&'static str, DisplayNodeConstructor>> = Lazy::new(|| {
    let mut m: HashMap<&'static str, fn() -> Box<dyn DisplayNodeKind>> = HashMap::new();

    m.insert("start", || Box::new( DisplayStartNode {} ));
    m.insert("number", || Box::new( DisplayNumberNode {} ));
    m.insert("addition", || Box::new( DisplayAdditionNode {} ));
    m.insert("text", || Box::new( DisplayTextNode {} ));
    m.insert("multiply", || Box::new( DisplayMultiplyNode {} ));
    m.insert("bool", || Box::new( DisplayBoolNode {} ));
    m.insert("print", || Box::new( DisplayPrintNode {} ));

    m
});

