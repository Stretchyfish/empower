
pub struct Settings
{
    pub developer_mode: bool,
    
}

impl Settings
{
    pub fn new() -> Self
    {
        Self
        {
            developer_mode: false,
            
        }
    }
}
