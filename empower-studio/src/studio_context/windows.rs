use crate::global_space::{DeveloperPanel, ExportPanel, ProjectNameWindow};

#[derive(Clone)]
pub struct Windows
{
    pub developer_panel: DeveloperPanel,
    pub project_name_panel: ProjectNameWindow, // @TODO, consider renaming it to panel
    pub export_panel: ExportPanel,
}

impl Windows
{
    pub fn new() -> Self
    {
        Self
        {
            developer_panel: DeveloperPanel::new(),
            project_name_panel: ProjectNameWindow::new(),
            export_panel: ExportPanel::new(),
        }
    }
}
