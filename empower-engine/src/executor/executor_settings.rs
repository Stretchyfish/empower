
#[derive(Clone)]
pub struct ExecutorSettings
{
    pub artificial_delay: Option<std::time::Duration>,
}

impl ExecutorSettings
{
    pub fn new() -> Self
    {
        Self
        {
            artificial_delay: None,
        }
    }

    pub fn new_debug_mode() -> Self
    {
        Self
        {
            artificial_delay: None,
        }
    }

    // pub fn clear_cache(&mut self)
    // {
    //     if let Some( outputs ) = &mut self.outputs
    //     {
    //         outputs.clear();
    //     }
    // }

    pub fn set_artificial_delay(&mut self, delay: f32)
    {
        if delay < 0.01 // To avoid floating point precision error, the check is done this way instead
        {
            self.artificial_delay = None;
            return;
        }

        self.artificial_delay = Some( std::time::Duration::from_secs_f32(delay) );
    }
}
