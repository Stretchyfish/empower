use serde::{Deserialize, Serialize};

use super::NodeKind;
use crate::{assets::{AssetId, AssetKind, Assets}, compiler::{CompilePlan, CompilePlanStep, Instruction, RegisterAddress}, node_graph::{NodeGraph, node::SettingsKind, port::{PortDefinition, PortDirection}}, project::ProjectState};

#[derive(Serialize, Deserialize, Clone)]
pub struct SubGraphNode
{
    pub graph_asset_id: Option<AssetId>,

    graph_start_node_output_port_definitions: Vec<PortDefinition>, // The output and input nodes are switched here due to the nature of the start en end node
    graph_end_node_input_port_definitions: Vec<PortDefinition>,
}

impl SubGraphNode
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

}

#[typetag::serde]
impl NodeKind for SubGraphNode
{
    fn name(&self) -> &str { "sub graph" }
    fn size(&self) -> egui::Vec2
    {
        if self.graph_asset_id.is_none()
        {
            return egui::vec2(300.0, 170.0);
        }

        let extra_vertical_offset = 40.0 * (std::cmp::max(self.graph_start_node_output_port_definitions.len(), self.graph_end_node_input_port_definitions.len()) as f32);
        egui::vec2(300.0, 220.0 + extra_vertical_offset)
    }

    fn settings(&mut self) -> Vec<SettingsKind<'_>>
    {
        let graph_id = self.graph_asset_id;
        let mut settings = vec![
            SettingsKind::Asset {
                label: "graph",
                value: &mut self.graph_asset_id,
                kind: AssetKind::NodeGraph,
        }];

        if let Some(graph_id) = graph_id
        {
            settings.push(SettingsKind::OpenGraph { graph_id });
        }

        settings
    }

    fn input_port_definitions(&self) -> Vec<PortDefinition>
    {
        self.graph_start_node_output_port_definitions.clone()
    }

    fn output_port_definitions(&self) -> Vec<PortDefinition>
    {
        self.graph_end_node_input_port_definitions.clone()
    }
    fn compile(&self, _: &Vec<RegisterAddress>, _: &Vec<RegisterAddress>, _: &Assets, _: &ProjectState) -> Result<CompilePlan, String>
    {
        let steps = match self.graph_asset_id
        {
            Some(graph_id) => vec![
                CompilePlanStep::QueueGraph(graph_id),
                CompilePlanStep::Emit(Instruction::CallGraph(graph_id))
            ],
            None => Vec::new(),
        };

        Ok(CompilePlan { steps })
    }
}

