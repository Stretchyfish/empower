use crate::global_space::DeveloperPanel;

pub struct Windows
{
    pub developer_panel: DeveloperPanel,
    
}

impl Windows
{
    pub fn new() -> Self
    {
        Self
        {
            developer_panel: DeveloperPanel::new(),
        }
    }
}
