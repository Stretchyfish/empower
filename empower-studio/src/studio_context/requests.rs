use std::path::PathBuf;

use empower_engine::NodeGraphKey;

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
    StartExecutionFrom { node_key: NodeGraphKey },
    StopExeuction,
}
