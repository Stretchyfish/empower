use std::collections::HashMap;
use std::collections::VecDeque;

use crate::assets::AssetId;

use super::CompiledGraph;
use super::Program;

pub struct CompilerContext
{
    graphs_to_compile: VecDeque<AssetId>,
    compiled_graphs: HashMap<AssetId, CompiledGraph>,
}

impl CompilerContext
{
    pub fn new_with_graph_to_compile(entry_graph: AssetId) -> Self
    {
        Self
        {
            graphs_to_compile: VecDeque::from(vec![ entry_graph ]),
            compiled_graphs: HashMap::new(),
        }
    }

    pub fn get_next_graph_to_compile(&mut self) -> Option<AssetId>
    {
        self.graphs_to_compile.pop_front()
    }

    pub fn add_more_graphs_to_build(&mut self, graph_ids: &Vec<AssetId>)
    {
        self.graphs_to_compile.extend(graph_ids);
    }

    pub fn add_compiled_graph(&mut self, graph_id: AssetId, graph: CompiledGraph)
    {
        self.compiled_graphs.insert(graph_id, graph);
    }

    pub fn get_program(&self, entry_graph: AssetId) -> Program
    {
        Program {
            entry_graph_id: entry_graph,
            compiled_graphs: self.compiled_graphs.clone()
        }
    }
}
