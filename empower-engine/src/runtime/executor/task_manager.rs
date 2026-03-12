use std::collections::{HashMap, HashSet};

use crate::{NodeGraphKey, utility::text_buffer::TextBuffer};

pub use super::task::{Task, Job, ShowJob, TaskId};

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct TaskManager
{
    pub tasks: HashMap<TaskId, Task>,
}

impl TaskManager
{
    pub fn new() -> Self
    {
        TaskManager
        {
            tasks: HashMap::new(),
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

    pub fn add_loop(&mut self, parent: &Job) -> TaskId
    {
        let new_task_id = self.create_task_with_parent( parent );

        let parrent_task = self.tasks.get_mut( &parent.task_id ).unwrap();

        if parrent_task.children.is_empty()
        {
            parrent_task.children.insert(parent.node_key, HashSet::new() );
        }

        parrent_task.children.get_mut( &parent.node_key ).unwrap().insert(new_task_id);

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

        let parent_task = self.tasks.get_mut(&removed_tasks_parent.task_id).unwrap();

        {
            let loops_created_from_node = parent_task.children.get_mut(&removed_tasks_parent.node_key).unwrap();

            loops_created_from_node.remove(task_id);

            if !loops_created_from_node.is_empty()
            {
                return;
            }
        }

        parent_task.children.remove(&removed_tasks_parent.node_key);

        self.remove_node_from_update(&removed_tasks_parent.task_id, &removed_tasks_parent.node_key);
    }

    pub fn continue_loop(&mut self, task_id: &TaskId, node_key: &NodeGraphKey) -> Option<TaskId>
    {
        let mut task_that_finished = None;

        {
            let task_executing = self.tasks.get_mut(task_id).unwrap();

            let tasks_created_from_node = task_executing.children.get_mut(node_key);

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
        let task_executing = self.tasks.get_mut(task_id).unwrap();
        task_executing.children.get_mut(node_key).unwrap().remove(&task_that_finished.unwrap());
        task_executing.children.get_mut(node_key).unwrap().insert(new_task_id);

        Some( new_task_id )
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

    pub fn add_node_to_show_key(&mut self, task_id: &TaskId, node_key: &NodeGraphKey, node_name: &str)
    {
        // @TODO, this whole function will have problems if the same window is triggered twice, be aware!

        let new_name = self.adjust_window_name(node_name); // Placed here for borrower satisfaction

        let task = self.tasks.get_mut(task_id);

        if task.is_none() // @TODO, there is a lot of these checks, consider if they are even needed
        {
            return;
        }

        let task = task.unwrap();
        task.nodes_to_update.insert(*node_key);
        task.nodes_to_show.insert(*node_key, new_name);
    }

    pub fn get_nodes_to_show(&self) -> Vec<ShowJob>
    {
        let mut nodes_to_show = Vec::new();

        for (task_id, task) in &self.tasks
        {
            let show_jobs_in_task = task.nodes_to_show.iter().map(|(node_key, title)| ShowJob::new(*task_id, node_key.clone(), title.clone()) );
            nodes_to_show.extend(show_jobs_in_task);
        }

        nodes_to_show
    }

    pub fn remove_node_from_show(&mut self, task_id: &TaskId, node_key: &NodeGraphKey)
    {
        let task = self.tasks.get_mut(task_id);

        if task.is_none()
        {
            return;
        }

        let task = task.unwrap();
        task.nodes_to_show.remove(node_key);
    }

    fn adjust_window_name(&self, node_name: &str) -> String
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
