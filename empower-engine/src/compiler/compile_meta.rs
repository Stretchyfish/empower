use std::collections::HashMap;

use crate::{assets::AssetId, compiler::InstructionAddress, node_graph::NodeGraphKey};

pub type InstructionGraphTraces = HashMap<InstructionAddress, NodeGraphKey>;
pub type InstructionTraces = HashMap<AssetId, InstructionGraphTraces>;

#[derive(Clone)]
pub struct CompileMeta
{
    pub trace: InstructionTraces,
}

impl CompileMeta
{
    pub fn new() -> Self
    {
        Self
        {
            trace: HashMap::new(),
        }
    }
}
