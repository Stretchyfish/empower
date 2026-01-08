use std::collections::HashMap;

use crate::runtime::analysis::detect_execution_order;
use crate::{NodeGraph, NodeGraphKey};


#[derive(Clone)]
pub struct LoopManager
{
    loops: HashMap<NodeGraphKey, Vec<NodeGraphKey>>,
    
}

impl LoopManager
{
    pub fn new() -> Self
    {
        Self
        {
            loops: HashMap::new(),
        }
    }

    pub fn add_loop(&mut self, node_key: &NodeGraphKey, node_graph: &NodeGraph)
    {
        let nodes_in_loop = detect_execution_order(node_key, node_graph);
        self.loops.insert(*node_key, nodes_in_loop);
    }

    pub fn check_for_loop_triggers(&mut self, )
    {

        
    }
}
