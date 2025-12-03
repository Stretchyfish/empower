
pub struct DisplayNode
{
    title: &'static str,
    
}

impl DisplayNode
{
    pub fn new(title: &'static str) -> Self
    {
        Self { title }
    }
}