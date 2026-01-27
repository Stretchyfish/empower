use std::collections::{HashMap, HashSet, VecDeque};
use crate::NodeGraphKey;

pub type TaskId = i32;

#[derive(Clone, Copy)]
pub struct Job
{
    pub task_id: TaskId,
    pub node_key: NodeGraphKey,
}

impl Job
{
    pub fn new(task_id: TaskId, node_key: NodeGraphKey) -> Self
    {
        Self
        {
            task_id,
            node_key,
        }
        
    }
}

#[derive(Clone)]
pub struct Task
{
    pub parent: Option<Job>,
    pub nodes_to_setup: VecDeque<NodeGraphKey>,
    pub nodes_to_update: HashSet<NodeGraphKey>,
    pub nodes_to_show: HashMap<NodeGraphKey, String>,
    pub sub_tasks: HashMap<NodeGraphKey, HashSet<TaskId>>,
}

impl Task
{
    pub fn new(parent: Option<Job>) -> Self
    {
        Self
        {
            parent,
            nodes_to_setup: VecDeque::new(),
            nodes_to_update: HashSet::new(),
            nodes_to_show: HashMap::new(),
            sub_tasks: HashMap::new(),
        }
    }
    
    pub fn is_finished(&self) -> bool
    {
        if self.nodes_to_setup.is_empty() && self.nodes_to_update.is_empty() && self.nodes_to_show.is_empty()
        {
            return true;
        }

        false
    }
}
