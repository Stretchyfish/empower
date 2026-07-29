use std::collections::HashMap;

use crate::{compiler::InstructionAddress, node_graph::NodeGraphKey};


#[derive(Clone)]
pub struct CompileMeta
{
    pub trace: HashMap<InstructionAddress, NodeGraphKey>,
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
