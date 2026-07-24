
#[derive(Clone)]
pub struct ExecutorSettings
{
    pub outputs: Option<Vec<String>>,
}

impl ExecutorSettings
{
    pub fn new() -> Self
    {
        Self
        {
            outputs: None,
        }
    }

    pub fn new_debug_mode() -> Self
    {
        Self
        {
            outputs: Some( Vec::new() ),
        }
    }

    pub fn clear_cache(&mut self)
    {
        if let Some( outputs ) = &mut self.outputs
        {
            outputs.clear();
        }
    }
}
