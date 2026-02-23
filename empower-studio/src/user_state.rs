
#[derive(Default, Clone, PartialEq, Eq)]
pub enum UserState
{
    #[default] Idle,
    DraggingAsset,
    NamingProject { new_project_name: String },
}
