use crate::global_space::{DeveloperPanel, ProjectNameWindow};

#[derive(Clone)]
pub struct Windows
{
    pub developer_panel: DeveloperPanel,
    pub project_name_panel: ProjectNameWindow, // @TODO, consider renaming it to panel
}

impl Windows
{
    pub fn new() -> Self
    {
        Self
        {
            developer_panel: DeveloperPanel::new(),
            project_name_panel: ProjectNameWindow::new(),
        }
    }
}
