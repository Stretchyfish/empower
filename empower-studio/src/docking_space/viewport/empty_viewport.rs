use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct EmptyViewport
{
    
}

impl EmptyViewport
{
    pub fn new() -> Self
    {
        Self
        {
            
        }
    }
}
