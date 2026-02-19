
#[derive(Default)]
pub struct UserInputs
{
    clicked_esp: bool,
}

pub fn get_user_inputs() -> UserInputs
{
    UserInputs::default()
}
