use serde::{Deserialize, Serialize};

use crate::{assets::AssetId, node_graph::{NodeGraph, port::{PortDefinition, PortDirection}}};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct SubGraphState
{
    pub graph_asset_id: Option<AssetId>,

    graph_start_node_output_port_definitions: Vec<PortDefinition>, // The output and input nodes are switched here due to the nature of the start en end node
    graph_end_node_input_port_definitions: Vec<PortDefinition>,
}

impl SubGraphState
{
    pub fn new() -> Self
    {
        Self
        {
            graph_asset_id: None,

            graph_start_node_output_port_definitions: Vec::new(), 
            graph_end_node_input_port_definitions: Vec::new(),
        }
    }

    pub fn from(graph_id: AssetId, graph: &NodeGraph) -> Self
    {
        let start_node_output_ports = graph.get_node_output_ports(graph.start_node_key);

        let end_node_input_ports = if graph.end_node_key.is_some()
        {
            graph.get_node_input_ports(graph.end_node_key.unwrap())
        }
        else
        {
            Vec::new()
        };

        let start_node_output_definitions: Vec<PortDefinition> = start_node_output_ports.iter().map(|i|
        {
            let mut port_definition = i.to_port_definitions();
            port_definition.direction = PortDirection::Input;
            port_definition
        }).collect();

        let end_node_input_definitions: Vec<PortDefinition> = end_node_input_ports.iter().map(|i|
        {
            let mut port_definition = i.to_port_definitions();
            port_definition.direction = PortDirection::Output;
            port_definition
        }).collect();
        
        Self
        {
            graph_asset_id: Some(graph_id),

            graph_start_node_output_port_definitions: start_node_output_definitions,
            graph_end_node_input_port_definitions: end_node_input_definitions,
        }
    }

    pub fn get_input_port_definitions(&self) -> Vec<PortDefinition>
    {
        self.graph_start_node_output_port_definitions.clone()
    }

    pub fn get_output_port_definitions(&self) -> Vec<PortDefinition>
    {
        self.graph_end_node_input_port_definitions.clone()
    }
}

