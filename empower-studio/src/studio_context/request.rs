use std::path::PathBuf;

use empower_engine::{assets::AssetId, distribution::ExportConfig};

use crate::docking_space::Viewport;

pub enum Request
{
    DefaultLayout,
    AddViewport { viewport: Viewport },
    AddOrFocusGraphViewport { graph_id: AssetId },
    AddViewportAtFirstLeaf { viewport: Viewport }, // @TODO, look into a better way of doing this?
    SaveStudio,
    LoadStudio,
    SaveProject,
    SaveProjectAs,
    LoadProject,
    LoadSpecificProject { project_path: PathBuf },
    Compile,
    StartExecute,
    StopExecute,
    ExportProject { config: ExportConfig },
    ImportAsset,
    StartDraggingAsset { asset_id: AssetId },
    StopDraggingAsset,
}
