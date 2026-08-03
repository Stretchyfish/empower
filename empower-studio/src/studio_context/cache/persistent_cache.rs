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
}
