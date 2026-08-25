use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq)]
pub enum ProjectState
{
    Temporary,
    Saved(PathBuf)
}

impl ProjectState
{
    pub fn to_string(&self) -> String
    {
        match self
        {
            ProjectState::Temporary => "Temporary".to_string(),
            ProjectState::Saved( path ) => format!("Saved ({})", path.to_string_lossy().to_string()),
        }
    }
}
