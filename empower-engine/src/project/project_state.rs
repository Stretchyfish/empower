use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum ProjectState
{
    Temporary,
    Saved
}
