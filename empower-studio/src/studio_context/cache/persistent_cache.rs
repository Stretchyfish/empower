use std::{collections::VecDeque, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct PersistentCache
{
    pub previous_projects: VecDeque<PathBuf>,
}

impl PersistentCache
{
    pub fn new() -> Self
    {
        Self
        {
            previous_projects: VecDeque::new(),
        }
    }

    pub fn add_previous_project(&mut self, project_path: PathBuf)
    {
        let mut project_index_to_remove = None;
        for (index, previous_project) in self.previous_projects.iter().enumerate()
        {
            if previous_project.to_string_lossy().to_string() == project_path.to_string_lossy().to_string()
            {
                project_index_to_remove = Some( index );
            }
        }

        if project_index_to_remove.is_some()
        {
            self.previous_projects.remove(project_index_to_remove.unwrap());
        }
        
        if self.previous_projects.len() > 10
        {
            self.previous_projects.pop_front();
        }

        self.previous_projects.push_back(project_path);
    }
}
