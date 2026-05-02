use std::collections::{HashMap, HashSet, VecDeque};
use crate::NodeGraphKey;

pub type TaskId = i32;

#[derive(Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct Job
{
    pub task_id: TaskId,
    pub node_key: NodeGraphKey,
    pub job_type: JobType
}

impl Job
{
    pub fn new(task_id: TaskId, node_key: NodeGraphKey, job_type: JobType) -> Self
    {
        Self
        {
            task_id,
            node_key,
            job_type
        }
        
    }
}

#[derive(Clone, Copy, serde::Serialize, serde::Deserialize)]
enum JobType
{
    Setup,
    Update,
    Show
}

#[derive(Clone)]
pub struct ShowJob
{
    pub task_id: TaskId,
    pub node_key: NodeGraphKey,
    pub window_title: String,
}

impl ShowJob
{
    pub fn new(task_id: TaskId, node_key: NodeGraphKey, window_title: String) -> Self
    {
        Self
        {
            task_id,
            node_key,
            window_title,
        }
    }
}
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Task
{
    pub parent: Option<Job>,
    pub children: HashMap<NodeGraphKey, HashSet<TaskId>>,
    pub nodes_to_setup: VecDeque<NodeGraphKey>,
    pub nodes_to_update: HashSet<NodeGraphKey>,
    pub nodes_to_show: HashMap<NodeGraphKey, String>,
}

impl Task
{
    pub fn new(parent: Option<Job>) -> Self
    {
        Self
        {
            parent,
            children: HashMap::new(),
            nodes_to_setup: VecDeque::new(),
            nodes_to_update: HashSet::new(),
            nodes_to_show: HashMap::new(),
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
