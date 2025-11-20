pub mod layout;
pub use layout::Layout;

pub struct StudioContext
{
    pub layout: Layout,
}

impl StudioContext
{
    pub fn new() -> Self
    {
        Self
        {
            layout: Layout::new(),
        }
    }
}