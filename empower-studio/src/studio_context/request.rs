use crate::user_state::{UserAction, UserState};

pub enum Request
{
    DefaultLayout,
    AddViewport { name: &'static str },
    SaveStudio,
    LoadStudio,
    UserStateClear,
    UserStateChange { layer_or_viewport: String, new_action: UserAction },
}
