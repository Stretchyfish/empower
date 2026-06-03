pub enum Request
{
    DefaultLayout,
    AddViewport { name: &'static str },
    SaveStudio,
    LoadStudio,
}
