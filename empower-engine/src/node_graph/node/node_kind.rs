use std::{collections::HashMap, num::{ParseFloatError, ParseIntError}};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::{assets::{AssetId, AssetKind}, compiler::{CompiledGraphContext, RegisterAddress}, node_graph::{Port, port::PortDefinition}, utility::alphabet_counter::AlphabetCounter, value::Value};

mod print_node;
use print_node::PrintNode;

mod start_node;
use start_node::StartNode;

mod number_node;
use number_node::NumberNode;

mod list_node;
use list_node::ListNode;

mod branch_node;
use branch_node::BranchNode;

mod wait_node;
use wait_node::WaitNode;

mod loop_node;
use loop_node::LoopNode;

mod sub_graph_node;
use sub_graph_node::SubGraphNode;

mod image_node;
use image_node::ImageNode;

mod show_image_node;
use show_image_node::ShowImageNode;

mod show_math_graph;
use show_math_graph::ShowMathGraph;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum NodeKind2
{
    Start,
    Print,
    Branch,
    Loop,
    Wait,
    List( ListState ),
    Image( ImageState ),
    ShowImage,
    MathGraph,
    SubGraph ( SubGraphState ),
}

impl NodeKind2
{
    pub fn name(&self) -> &'static str
    {
        match self
        {
            NodeKind2::Start => "start",
            NodeKind2::Print => "print",
            NodeKind2::Branch => "branch",
            NodeKind2::Loop => "loop",
            NodeKind2::Wait => "wait",
            NodeKind2::List(_) => "list",
            NodeKind2::Image(_) => "image",
            NodeKind2::ShowImage => "show image",
            NodeKind2::MathGraph => "math graph",
            NodeKind2::SubGraph(_) => "math graph",
        }
    }

    pub fn size(&self) -> egui::Vec2
    {
        match self
        {
            NodeKind2::Start => egui::vec2(300.0, 220.0),
            NodeKind2::Print => egui::vec2(300.0, 220.0),
            NodeKind2::Branch => egui::vec2(300.0, 220.0),
            NodeKind2::Loop => egui::vec2(300.0, 220.0),
            NodeKind2::Wait => egui::vec2(300.0, 220.0),
            NodeKind2::List(_) => egui::vec2(300.0, 220.0),
            NodeKind2::Image(_) => egui::vec2(300.0, 220.0),
            NodeKind2::ShowImage => egui::vec2(300.0, 220.0),
            NodeKind2::MathGraph => egui::vec2(300.0, 220.0),
            NodeKind2::SubGraph(_) => egui::vec2(300.0, 220.0),
        }
    }

    pub fn input_port_definitions(&self) -> Vec<PortDefinition>
    {
        match self
        {
            NodeKind2::Start => Vec::new(),
            NodeKind2::Print => vec![
                                        PortDefinition::new_input_execution_port(),
                                        PortDefinition::new_input_data_port("value".to_string(), vec![ Value::Integer(0), Value::Float(0.0)]) ],
            NodeKind2::Branch => vec![
                                        PortDefinition::new_input_execution_port(),
                                        PortDefinition::new_input_data_port("a".to_string(), vec![ Value::Bool(false) ]) ],
            NodeKind2::Loop => vec![
                                        PortDefinition::new_input_execution_port() ],
            NodeKind2::Wait => vec![
                                        PortDefinition::new_input_execution_port(),
                                        PortDefinition::new_input_data_port("seconds".to_string(), vec![Value::Float(1.0)]) ],
            NodeKind2::List( state ) => state.get_input_port_definitions(),
            NodeKind2::Image( _ ) => Vec::new(),
            NodeKind2::ShowImage => vec![
                                            PortDefinition::new_input_execution_port(),
                                            PortDefinition::new_input_data_port("image".to_string(), vec![ Value::Image( None ) ])
                                        ],
            NodeKind2::MathGraph => vec![
                                            PortDefinition::new_input_execution_port(),
                                            PortDefinition::new_input_data_port("math graph".to_string(), vec![ Value::List( Vec::new() ) ])
                                        ],
            NodeKind2::SubGraph(_) => Vec::new(),
        }
    }

    pub fn output_port_definitions(&self) -> Vec<PortDefinition>
    {
        match self
        {
            NodeKind2::Start => vec![ PortDefinition::new_output_execution_port() ],
            NodeKind2::Print => Vec::new(),
            NodeKind2::Branch => vec![
                                        PortDefinition::new_output_execution_port(),
                                        PortDefinition::new_output_execution_port() ],
            NodeKind2::Loop => vec![
                                        PortDefinition::new_output_execution_port() ],
            NodeKind2::Wait => vec![
                                        PortDefinition::new_output_execution_port() ],
            NodeKind2::List( state ) => state.get_output_port_definitions(),
            NodeKind2::Image( state ) => state.get_output_port_definitions(),
            NodeKind2::ShowImage => Vec::new(),
            NodeKind2::MathGraph => Vec::new(),
            NodeKind2::SubGraph(_) => Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct ListState
{
    pub value_type: Value,
    pub size: usize,
}

impl ListState
{
    pub fn new() -> Self
    {
        Self
        {
            value_type: Value::Integer(0),
            size: 2,
        }
    }

    pub fn get_input_port_definitions(&self) -> Vec<PortDefinition>
    {
        let mut inputs = Vec::with_capacity(self.size);

        let base_value = &self.value_type;

        let mut alphabet_counter = AlphabetCounter::new();

        for _ in 0..self.size
        {
            let letter = alphabet_counter.next_letter().to_string();
            inputs.push( PortDefinition::new_input_data_port(letter, vec![ base_value.clone() ]) );
        }
        
        inputs
    }

    pub fn get_output_port_definitions(&self) -> Vec<PortDefinition>
    {
        vec![
            PortDefinition::new_output_data_port("".to_string(), vec![ Value::List( Vec::new() ) ])
        ]
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct ImageState
{
    pub image_asset_id: Option<AssetId>,
}

impl ImageState
{
    pub fn new() -> Self
    {
        Self
        {
            image_asset_id: None,
        }
    }

    pub fn get_output_port_definitions(&self) -> Vec<PortDefinition>
    {
        if self.image_asset_id.is_none()
        {
            return Vec::new();
        }

        vec![
            PortDefinition::new_output_data_port("image".to_string(), vec![ Value::Image( self.image_asset_id ) ])
        ]
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct SubGraphState
{
    pub graph_asset_id: Option<AssetId>,
}

impl SubGraphState
{
    pub fn new() -> Self
    {
        Self
        {
            graph_asset_id: None,
        }
    }
}

#[typetag::serde(tag="node_kind")]
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

    fn sync_node_edit(&mut self, index: usize) -> NodeSyncResponse;

    fn sync_node_state(&mut self, node_state: NodeState);

    fn compile(&self, ctx: &mut CompiledGraphContext, input_port_register_adresses: Vec<RegisterAddress>, output_port_register_adresses: Vec<RegisterAddress>);

    fn control_flow(&self) -> ControlFlowKind;
}

impl Clone for Box<dyn NodeKind>
{
    fn clone(&self) -> Self
    {
        self.clone_box()
    }
}

pub enum LoopSettings
{
    Forever,
    Interval,
}

pub enum ControlFlowKind
{
    None,
    Linear,
    Branch,
    Loop ( LoopSettings ),
}

#[derive(Clone, Serialize, Deserialize)]
pub enum NodeEdit
{
    Text { label: String, text: String, parseble: bool },
    CheckBox { toggle: bool },
    AssetSelector { asset_id: Option<AssetId>, kind: AssetKind },
    GraphViewportOpener { graph_id: Option<AssetId>},
    EnumBox { label: String, states: Vec<String>, current_state: String },
}

impl NodeEdit
{
    pub fn parse_to_usize(&mut self) -> Result<usize, ParseIntError>
    {
        let (text, parse) = match self
        {
            NodeEdit::Text { label: _, text, parseble } => ( text, parseble ),
            _ => panic!("list node has an invalid node edit")
        };

        let parsed_value = text.parse::<usize>();

        *parse = parsed_value.is_ok();

        parsed_value
    }

    pub fn parse_to_i32(&mut self) -> Result<i32, ParseIntError>
    {
        let (text, parse) = match self
        {
            NodeEdit::Text { label: _, text, parseble } => ( text, parseble ),
            _ => panic!("list node has an invalid node edit")
        };

        let parsed_value = text.parse::<i32>();

        *parse = parsed_value.is_ok();

        parsed_value
    }

    pub fn parse_to_f32(&mut self) -> Result<f32, ParseFloatError>
    {
        let (text, parse) = match self
        {
            NodeEdit::Text { label: _, text, parseble } => ( text, parseble ),
            _ => panic!("list node has an invalid node edit")
        };

        let parsed_value = text.parse::<f32>();

        *parse = parsed_value.is_ok();

        parsed_value
    }
}

pub enum NodeSyncResponse
{
    Nothing,
    NodesStructureChanged,
    LoadSubgraph( AssetId ),
}

pub enum NodeState
{
    GraphStartAndEndPorts { start_input_ports: Vec<Port>, end_output_ports: Vec<Port>},
}

type NodeConstructor = fn() -> Box<dyn NodeKind>;

pub static NODE_KIND_REGISTRY: Lazy<HashMap<&'static str, NodeConstructor>> = Lazy::new(|| {
    let mut r: HashMap<&'static str, NodeConstructor> = HashMap::new();

    r.insert( PrintNode::new().name(), || PrintNode::new());
    r.insert( StartNode::new().name(), || StartNode::new());
    r.insert( NumberNode::new().name(), || NumberNode::new());
    r.insert( ListNode::new().name(), || ListNode::new());
    r.insert( BranchNode::new().name(), || BranchNode::new());
    r.insert( WaitNode::new().name(), || WaitNode::new());
    r.insert( LoopNode::new().name(), || LoopNode::new());
    r.insert( SubGraphNode::new().name(), || SubGraphNode::new());
    r.insert( ImageNode::new().name(), || ImageNode::new());
    r.insert( ShowImageNode::new().name(), || ShowImageNode::new());
    r.insert( ShowMathGraph::new().name(), || ShowMathGraph::new());

    r
});
