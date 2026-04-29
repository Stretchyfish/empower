
#[derive(Default)]
pub struct UserInputs
{
    pub mouse_position: egui::Pos2,
    pub clicked_primary_mouse_button: bool,
    pub holding_primary_mouse_button: bool,
    pub released_primary_mouse_button: bool,
    pub clicked_secondary_mouse_button: bool,
    pub holding_secondary_mouse_button: bool,
    pub released_secondary_mouse_button: bool,
    pub clicked_esp: bool,
    pub holding_ctrl: bool,
    pub holding_alt: bool,
    pub clicked_s: bool,
    pub clicked_d: bool,
    pub clicked_enter: bool,
    pub clicked_up_arrow: bool,
    pub clicked_down_arrow: bool,
}

pub fn get_user_inputs(ctx: &egui::Context) -> UserInputs
{
    let mut user_inputs = UserInputs::default();

    user_inputs.mouse_position = ctx.input(|i| i.pointer.latest_pos().unwrap_or_default() );
    user_inputs.clicked_primary_mouse_button = ctx.input(|i| i.pointer.primary_clicked() );
    user_inputs.holding_primary_mouse_button = ctx.input(|i| i.pointer.primary_pressed() );
    user_inputs.released_primary_mouse_button = ctx.input(|i| i.pointer.primary_released() );
    user_inputs.clicked_secondary_mouse_button = ctx.input(|i| i.pointer.secondary_clicked() );
    user_inputs.holding_secondary_mouse_button = ctx.input(|i| i.pointer.secondary_pressed() );
    user_inputs.released_secondary_mouse_button= ctx.input(|i| i.pointer.secondary_released() );
    user_inputs.clicked_esp = ctx.input(|i| { i.key_pressed(egui::Key::Escape) });
    user_inputs.holding_ctrl = ctx.input(|i| { i.modifiers.ctrl });
    user_inputs.holding_alt = ctx.input(|i| { i.modifiers.alt });
    user_inputs.clicked_s = ctx.input(|i| { i.key_pressed(egui::Key::S) });
    user_inputs.clicked_d = ctx.input(|i| { i.key_pressed(egui::Key::D) });
    user_inputs.clicked_enter = ctx.input(|i| { i.key_pressed(egui::Key::Enter) });
    user_inputs.clicked_up_arrow =  ctx.input(|i| { i.key_pressed(egui::Key::ArrowUp) });
    user_inputs.clicked_down_arrow =  ctx.input(|i| { i.key_pressed(egui::Key::ArrowDown) });

    user_inputs
}
