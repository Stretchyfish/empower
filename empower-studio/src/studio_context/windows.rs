use crate::global_space::{AssetImportDialog, DeveloperPanel, ExportPanel, ProjectLoadDialog, ProjectNameWindow};

#[derive(Clone)]
pub struct Windows
{
    pub developer_panel: DeveloperPanel,
    pub project_name_panel: ProjectNameWindow, // @TODO, consider renaming it to panel
    pub export_panel: ExportPanel,
    pub asset_import_dialog: AssetImportDialog,
    pub project_load_dialog: ProjectLoadDialog,
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
            asset_import_dialog: AssetImportDialog::new(),
            project_load_dialog: ProjectLoadDialog::new(),
        }
    }
}
