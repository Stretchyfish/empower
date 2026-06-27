use crate::{docking_space::Viewport, user_state::UserAction};

pub enum Request
{
    DefaultLayout,
    AddViewport { viewport: Viewport },
    AddViewportAtFirstLeaf { viewport: Viewport }, // @TODO, look into a better way of doing this?
    SaveStudio,
    LoadStudio,
    UserStateClear,
    UserStateChange { layer_or_viewport: String, new_action: UserAction },
    Compile,
    StartExecute,
    StopExecute,
}
