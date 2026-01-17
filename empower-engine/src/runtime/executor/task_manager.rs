use std::{collections::{HashMap, HashSet, VecDeque}, fmt::Debug};

use crate::{NodeGraphKey, node_graph::node::node_kind::LoopNode};

use super::task::Task;

pub type TaskId = i32;

#[derive(Clone)]
pub struct TaskManager
{
    pub tasks: HashMap<TaskId, Task>,
    pub loops: HashMap<NodeGraphKey, HashSet<TaskId>>,
}

impl TaskManager
{
    pub fn new() -> Self
    {
        TaskManager
        {
            tasks: HashMap::new(),
            loops: HashMap::new(),
        }
    }

    pub fn add_task(&mut self) -> TaskId
    {
        let task_id = self.tasks.len() as TaskId;

        let new_task = Task
                        {
                            nodes_to_setup: VecDeque::new(),
                            nodes_to_update: HashSet::new(),
                                
                        };

        self.tasks.insert(task_id, new_task);

        task_id
    }

    pub fn add_loop(&mut self, node_key: &NodeGraphKey) -> TaskId
    {
        if !self.loops.contains_key(node_key)
        {
            self.loops.insert(*node_key, HashSet::new());
        }

        let task_id = self.add_task();

        let loop_instance = self.loops.get_mut(node_key).unwrap();
        loop_instance.insert(task_id);

        task_id
    }

    pub fn continue_loop(&mut self, node_key: &NodeGraphKey) -> Option<TaskId>
    {
        let loop_instance = self.loops.get(node_key);
        if loop_instance.is_none() // This should in theory never happen, but is placed for safety
        {
            return None;
        }

        let loop_instance = loop_instance.unwrap().clone();

        for task_id in loop_instance.iter()
        {
            if self.is_task_finished(&task_id)
            {
                let new_task_id = self.add_task();
                return Some( new_task_id ); // @TODO, this approach has the potential to be dangerous two loops finshed in the same step
            }
        }

        None
    }

    pub fn add_node_key(&mut self, task_id: &TaskId, node_key: &NodeGraphKey)
    {
        let task = self.tasks.get_mut(task_id);

        if task.is_none()
        {
            return;
        }

        let task = task.unwrap();
        task.nodes_to_setup.push_back(*node_key);
    }

    pub fn add_nodes_to_setup_keys(&mut self, task_id: &TaskId, node_keys: &Vec<NodeGraphKey>)
    {
        let task = self.tasks.get_mut(task_id);

        if task.is_none()
        {
            return;
        }

        let task = task.unwrap();
        task.nodes_to_setup.extend(node_keys);
    }

    pub fn add_node_to_update_key(&mut self, task_id: &TaskId, node_key: &NodeGraphKey)
    {
        let task = self.tasks.get_mut(task_id);

        if task.is_none()
        {
            return;
        }

        let task = task.unwrap();
        task.nodes_to_update.insert(*node_key);
    }

    pub fn remove_node_from_update(&mut self, task_id: &TaskId, node_key: &NodeGraphKey)
    {
        let task = self.tasks.get_mut(task_id);

        if task.is_none()
        {
            return;
        }

        let task = task.unwrap();
        task.nodes_to_update.remove(node_key);
    }

    pub fn cleanup_tasks(&mut self)
    {
        let mut task_to_remove = None;

        for task_id in self.tasks.keys()
        {
            if self.is_task_finished(task_id)
            {
                task_to_remove = Some( task_id.clone() );
                break;
            }
        }

        if task_to_remove.is_none()
        {
            return;
        }

        self.tasks.remove(&task_to_remove.unwrap());
    }

    fn is_task_finished(&self, task_id: &TaskId) -> bool
    {
        let task = self.tasks.get(task_id);

        if task.is_none()
        {
            return true;
        }

        let task = task.unwrap();

        if task.nodes_to_setup.is_empty() && task.nodes_to_update.is_empty()
        {
            return true;
        }

        false
    }
    // pub fn add_nodes_to_update_keys(&mut self)

    pub fn remove_task(&mut self)
    {
        
    }

    // @TODO, in the future, consider rewritting this to be only send one node
    pub fn get_next_nodes_to_setup(&mut self) -> Vec<Job>
    {
        let mut nodes_to_setup = Vec::new();

        for (task_id, task) in &mut self.tasks
        {
            let next_node_to_setup = task.nodes_to_setup.pop_front();

            if next_node_to_setup.is_none()
            {
                continue;
            }

            nodes_to_setup.push( Job { task_id: *task_id, node_key: next_node_to_setup.unwrap() } );
        }

        nodes_to_setup
    }

    pub fn get_next_nodes_to_update(&mut self) -> Vec<Job>
    {
        let mut nodes_to_update = Vec::new();

        for (task_id, task) in &mut self.tasks
        {
            let jobs_in_task = task.nodes_to_update.iter().map(|node_key| Job { task_id: *task_id, node_key: *node_key } );
            nodes_to_update.extend( jobs_in_task );
        }

        nodes_to_update
    }

    pub fn has_tasks(&self) -> bool
    {
        !self.tasks.is_empty()
    }
}

pub struct Job
{
    pub task_id: TaskId,
    pub node_key: NodeGraphKey,
}
