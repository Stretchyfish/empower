
#[derive(Default)]
pub struct UserInputs
{
    pub clicked_esp: bool,
    pub holding_ctrl: bool,
    pub clicked_s: bool,
}

pub fn get_user_inputs(ctx: &egui::Context) -> UserInputs
{
    let mut user_inputs = UserInputs::default();

    user_inputs.clicked_esp = ctx.input(|i| { i.key_pressed(egui::Key::Escape) });
    user_inputs.holding_ctrl = ctx.input(|i| { i.modifiers.ctrl });
    user_inputs.clicked_s = ctx.input(|i| { i.key_pressed(egui::Key::S) });

    user_inputs
}
