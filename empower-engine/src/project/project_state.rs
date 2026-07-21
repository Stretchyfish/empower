use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum ProjectState
{
    Temporary,
    Saved
}

impl ProjectState
{
    pub fn to_string(&self) -> String
    {
        match self
        {
            ProjectState::Temporary => "Temporary".to_string(),
            ProjectState::Saved => "Saved".to_string(),
        }
    }
}
