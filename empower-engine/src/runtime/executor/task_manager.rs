use std::{collections::{HashMap, HashSet, VecDeque}, fmt::Debug};

use crate::{NodeGraphKey, node_graph::node::node_kind::LoopNode, utility::text_buffer::TextBuffer};

pub use super::task::{Task, Job, TaskId};

#[derive(Clone)]
pub struct TaskManager
{
    pub tasks: HashMap<TaskId, Task>,
    pub loops: HashMap<NodeGraphKey, HashSet<TaskId>>,
    pub loops_2: HashMap<TaskId, NodeGraphKey>,
    pub loop_manager: LoopManager,
    pub relations: HashMap<NodeGraphKey, HashSet<TaskId>>,
    pub main_window_key: Option<NodeGraphKey>, 
    pub windows: HashMap<TaskId, HashMap<NodeGraphKey, String>>,
    // @TODO, think about adding mapping, so you can have multiple of same node running
}

impl TaskManager
{
    pub fn new() -> Self
    {
        TaskManager
        {
            tasks: HashMap::new(),
            loops: HashMap::new(),
            loops_2: HashMap::new(),
            loop_manager: LoopManager::new(),
            relations: HashMap::new(),
            main_window_key: None,
            windows: HashMap::new(),
        }
    }

    pub fn create_task(&mut self) -> TaskId
    {
        let task_id = self.tasks.len() as TaskId;

        let new_task = Task::new(None);
        self.tasks.insert(task_id, new_task);

        task_id
    }

    pub fn create_task_with_parent(&mut self, parent: &Job) -> TaskId
    {
        let task_id = self.tasks.len() as TaskId;

        let new_task = Task::new(Some( *parent ));
        self.tasks.insert(task_id, new_task);

        task_id
    }

    pub fn create_task_with_task_id_and_parent(&mut self, task_id: &TaskId, parent: &Job)
    {
        let new_task = Task::new(Some( *parent ));
        self.tasks.insert(*task_id, new_task);
    }

    pub fn add_loop(&mut self, parent: &Job) -> TaskId
    {
        let new_task_id = self.create_task_with_parent( parent );

        if self.relations.is_empty()
        {
            self.relations.insert(parent.node_key, HashSet::new() );
        }

        self.relations.get_mut(&parent.node_key).unwrap().insert(new_task_id);

        new_task_id // Not really needed, but unsed for debugging
    }

    pub fn stop_task(&mut self, task_id: &TaskId)
    {
        self.tasks.remove(task_id);
    }

    pub fn remove_loop(&mut self, task_id: &TaskId)
    {
        let removed_task = self.tasks.remove(task_id);

        let removed_tasks_parent = removed_task.unwrap().parent.unwrap(); // @TODO, uff, thats dangerous

        {
            let loops_created_from_node = self.relations.get_mut(&removed_tasks_parent.node_key).unwrap();

            loops_created_from_node.remove(task_id);

            if !loops_created_from_node.is_empty()
            {
                return;
            }
        }

        self.relations.remove(&removed_tasks_parent.node_key);

        self.remove_node_from_update(&removed_tasks_parent.task_id, &removed_tasks_parent.node_key);
    }

    pub fn continue_loop_2(&mut self, task_id: &TaskId, node_key: &NodeGraphKey) -> Option<TaskId>
    {
        let mut task_that_finished = None;

        {
            let tasks_created_from_node = self.relations.get_mut(node_key);

            if tasks_created_from_node.is_none()
            {
                panic!("Tried to continue loop, but the loop is not registered in relations");
            }

            let tasks_created_from_node = tasks_created_from_node.unwrap().clone();

            for loop_task_id in tasks_created_from_node
            {
                let task = self.tasks.get(&loop_task_id );

                if task.is_none()
                {
                    task_that_finished = Some( loop_task_id );
                    break;
                }
            }
        }

        if task_that_finished.is_none()
        {
            return None;
        }

        let new_task_id = self.create_task_with_parent(&Job::new(*task_id, *node_key));

        // This behavior is slightly weird, but seems nessesary
        self.relations.get_mut(node_key).unwrap().remove(&task_that_finished.unwrap());
        self.relations.get_mut(node_key).unwrap().insert(new_task_id);

        Some( new_task_id )
    }

    // pub fn continue_loop(&mut self, node_key: &NodeGraphKey) -> Option<TaskId>
    // {
    //     let loop_instance = self.loops.get(node_key);
    //     if loop_instance.is_none() // This should in theory never happen, but is placed for safety
    //     {
    //         return None;
    //     }

    //     let loop_instance = loop_instance.unwrap().clone();

    //     for task_id in loop_instance.iter()
    //     {
    //         if self.is_task_finished(&task_id)
    //         {
    //             let new_task_id = self.add_task();
    //             return Some( new_task_id ); // @TODO, this approach has the potential to be dangerous two loops finshed in the same step
    //         }
    //     }

    //     None
    // }

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

    pub fn add_node_to_show_key(&mut self, task_id: &TaskId, node_key: &NodeGraphKey, node_name: &str)
    {
        // @TODO, this whole function will have problems if the same window is triggered twice, be aware!

        let new_name = self.adjust_window_name_2(node_name); // Placed here for borrower satisfaction

        let task = self.tasks.get_mut(task_id);

        if task.is_none() // @TODO, there is a lot of these checks, consider if they are even needed
        {
            return;
        }

        let task = task.unwrap();
        task.nodes_to_update.insert(*node_key);
        task.nodes_to_show.insert(*node_key, new_name);
    }

    pub fn create_window(&mut self, task_id: &TaskId, node_key: &NodeGraphKey, node_name: &str)
    {
        let new_name = self.adjust_window_name(node_name); // Placed here for borrower satisfaction
        if !self.windows.contains_key(task_id)
        {
            self.windows.insert(*task_id, HashMap::new());
        }

        let windows_in_task = self.windows.get_mut(task_id).unwrap();

        if windows_in_task.contains_key(node_key)
        {
            return;
        }

        if self.main_window_key == None
        {
            self.main_window_key = Some ( *node_key );
        }

        windows_in_task.insert(*node_key, new_name);
    }

    pub fn get_windows(&self) -> HashMap<TaskId, HashMap<NodeGraphKey, String>> // @TODO, improve naming
    {
        self.windows.clone()
    }

    pub fn remove_window(&mut self, task_id: &TaskId, node_key: &NodeGraphKey)
    {
        if *node_key == self.main_window_key.unwrap_or(0) // This should never possibly fail, but set to 0 for safety
        {
            self.main_window_key = None;
        }

        if !self.windows.contains_key(task_id) // Should never happen, but done for safety
        {
            return;
        }

        let task_windows_is_empty;

        {
            let task_windows = self.windows.get_mut(task_id).unwrap();
            task_windows.remove(node_key);

            task_windows_is_empty = task_windows.is_empty();
        }

        if task_windows_is_empty
        {
            self.windows.remove(node_key);
        }
    }

    pub fn clear_windows(&mut self)
    {
        self.main_window_key = None;
        self.windows.clear();
    }

    fn adjust_window_name_2(&self, node_name: &str) -> String
    {
        let mut number_of_windows_with_same_name = 0;
        for (_, task) in &self.tasks
        {
            for (_, window_title) in &task.nodes_to_show
            {
                if window_title.contains(node_name)
                {
                    number_of_windows_with_same_name += 1;
                }
            }
        }

        if number_of_windows_with_same_name == 0
        {
            return String::from( node_name );
        }

        format!("{} ({})", node_name, number_of_windows_with_same_name)
    }

    fn adjust_window_name(&self, node_name: &str) -> String
    {
        let mut number_of_windows_with_same_name = 0;
        for tasks in self.windows.values()
        {
            for text in tasks.values()
            {
                if text.contains(node_name)
                {
                    number_of_windows_with_same_name += 1;
                }
            }
        }

        if number_of_windows_with_same_name == 0
        {
            return String::from( node_name );
        }

        format!("{} ({})", node_name, number_of_windows_with_same_name)
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

    pub fn cleanup_finished_tasks(&mut self, execution_history: &mut TextBuffer)
    {
        let mut task_to_remove = None;

        for (task_id, task) in &self.tasks
        {
            if task.is_finished()
            {
                task_to_remove = Some( *task_id );
                break;
            }
        }

        if task_to_remove.is_none()
        {
            return;
        }

        self.tasks.remove(&task_to_remove.unwrap());
        execution_history.add_line(&format!("Removed task ({})", task_to_remove.unwrap()));
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

    pub fn get_tasks(&self) -> HashMap<TaskId, Task> // @TODO, think this cloning can be avoided
    {
        self.tasks.clone()
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

#[derive(Clone)]
pub struct LoopManager
{
    pub loop_task_ids: HashMap<TaskId, NodeGraphKey>,
    pub loop_root_nodes: HashMap<NodeGraphKey, HashSet<TaskId>>,
}

impl LoopManager
{
    pub fn new() -> Self
    {
        Self
        {
            loop_task_ids: HashMap::new(),
            loop_root_nodes: HashMap::new(),
        }
    }

    pub fn add_task(&mut self, task_id: &TaskId, node_key: &NodeGraphKey)
    {
        self.loop_task_ids.insert(*task_id, *node_key);

        if !self.loop_root_nodes.contains_key(node_key)
        {
            self.loop_root_nodes.insert(*node_key, HashSet::new() );
        }

        self.loop_root_nodes.get_mut(node_key).unwrap().insert(*task_id);
    }

    pub fn remove_task(&mut self, task_id: &TaskId)
    {
        let loop_root_key = self.loop_task_ids.remove(task_id);

        if loop_root_key.is_none() // This should only happen if a wrong key has been put in!
        {
            return;
        }

        let mut delete_root_node_from = false;

        {
            let loop_root_node = self.loop_root_nodes.get_mut(&loop_root_key.unwrap());
            if loop_root_node.is_none()
            {
                return;
            }

            let loop_root_node = loop_root_node.unwrap();

            loop_root_node.remove(task_id);

            if loop_root_node.is_empty()
            {
                delete_root_node_from = true;
            }
        }

        if delete_root_node_from
        {
            self.loop_root_nodes.remove(&loop_root_key.unwrap());
        }
    }

    pub fn get_tasks_created_from_node_key(&self, node_key: &NodeGraphKey) -> HashSet<TaskId>
    {
        let tasks_created_from_node_key = self.loop_root_nodes.get(node_key);

        if tasks_created_from_node_key.is_none()
        {
            println!("Node key searching {}", node_key);
            panic!();
        }

        tasks_created_from_node_key.unwrap().clone()
    }
}
