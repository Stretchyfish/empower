use crate::{docking_space::Viewport, user_state::UserAction};

pub enum Request
{
    DefaultLayout,
    AddViewport { viewport: Viewport },
    SaveStudio,
    LoadStudio,
    UserStateClear,
    UserStateChange { layer_or_viewport: String, new_action: UserAction },
    Compile,
    StartExecute,
    StopExecute,
}
