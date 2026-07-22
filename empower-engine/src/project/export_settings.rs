use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ExportSettings
{
    pub export_name: String,
    
}

impl ExportSettings
{
    pub fn new() -> Self
    {
        Self
        {
            export_name: String::new(),
            
        }
    }
}
