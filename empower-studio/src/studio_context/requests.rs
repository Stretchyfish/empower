use std::path::PathBuf;


pub enum Request
{
    Save,
    SaveLayout,
    LoadLayout,
    NewLayout,
    DefaultLayout,
    SaveProject,
    SaveProjectAs,
    LoadProject,
    LoadSpecificProject { project_path: PathBuf },
    AddViewport { name: &'static str },
    StartExecution,
    StopExeuction,
}
