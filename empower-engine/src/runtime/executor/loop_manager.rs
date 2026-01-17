use std::collections::{HashMap, HashSet};

use crate::runtime::analysis::detect_execution_order;
use crate::{NodeGraph, NodeGraphKey};

use super::TaskId;

#[derive(Clone)]
pub struct LoopManager
{
    loops: HashMap<NodeGraphKey, HashSet<TaskId>>,
    
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

    pub fn add_loop(&mut self, node_key: &NodeGraphKey, task_id: &TaskId)
    {
        if !self.loops.contains_key(node_key)
        {
            self.loops.insert(*node_key, HashSet::new() );
        }

        let loop_to_add = self.loops.get_mut(node_key);

        loop_to_add.unwrap().insert(*task_id);
    }

    pub fn check_for_loop_triggers(&mut self, )
    {

        
    }
}
