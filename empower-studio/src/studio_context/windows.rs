mod project_name_window;
use project_name_window::ProjectNameWindow;

#[derive(Clone)]
pub struct Windows // @TODO, consider a better name, maybe pop ups?
{
    pub project_name_window: ProjectNameWindow,
}

impl Windows
{
    pub fn new() -> Self
    {
        Self
        {
            project_name_window: ProjectNameWindow::new(),
        }
    }
}
