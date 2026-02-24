
pub enum Request
{
    Save,
    SaveLayout,
    LoadLayout,
    NewLayout,
    DefaultLayout,
    SaveProject,
    LoadProject,
    AddViewport { name: &'static str },
}
