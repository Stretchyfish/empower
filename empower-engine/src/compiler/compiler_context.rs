use std::collections::HashMap;
use std::collections::VecDeque;

use crate::compiler::Instruction;
use crate::{assets::AssetId, node_graph::NodeGraphKey};

use super::CompiledGraph;
use super::Program;

pub struct CompilerContext
{
    graphs_to_compile: VecDeque<AssetId>,
    graph_compiling: Option<CompiledGraph>,
    compiled_graphs: HashMap<AssetId, CompiledGraph>,
}

impl CompilerContext
{
    pub fn new_with_graph_to_compile(entry_graph: AssetId) -> Self
    {
        Self
        {
            graph_compiling: None,
            graphs_to_compile: VecDeque::from(vec![ entry_graph ]),
            compiled_graphs: HashMap::new(),
        }
    }

    pub fn get_next_graph_to_compile(&mut self) -> Option<AssetId>
    {
        self.graphs_to_compile.pop_front()
    }

    pub fn begin_compiling_new_graph(&mut self)
    {
        self.graph_compiling = Some( CompiledGraph::new() );
    }

    pub fn finish_compiling_current_graph(&mut self, graph_id: AssetId)
    {
        let mut graph_compiling = self.graph_compiling.take().unwrap();
        graph_compiling.instructions.push(Instruction::Return);

        self.compiled_graphs.insert(graph_id, graph_compiling);
    }

    pub fn get_current_compiling_graph(&mut self) -> &mut CompiledGraph
    {
        self.graph_compiling.as_mut().unwrap()
    }

    pub fn add_instruction_to_current_graph(&mut self, instruction: Instruction)
    {
        self.graph_compiling.as_mut().unwrap().instructions.push(instruction);
    }

    pub fn add_graph_to_compile(graph: AssetId)
    {
        
    }

    pub fn get_program(&self, entry_graph: AssetId) -> Program
    {
        Program {
            entry_graph_id: entry_graph,
            compiled_graphs: self.compiled_graphs.clone()
        }
    }
}
