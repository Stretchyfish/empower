use std::path::PathBuf;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct PreviousProject
{
    pub project_name: String,
    pub project_location: PathBuf,
}
