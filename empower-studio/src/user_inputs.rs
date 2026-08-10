
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
    pub holding_shift: bool,
    pub holding_alt: bool,
    pub clicked_a: bool,
    pub clicked_s: bool,
    pub clicked_d: bool,
    pub clicked_enter: bool,
    pub clicked_up_arrow: bool,
    pub clicked_down_arrow: bool,
    pub clicked_delete: bool,
    pub clicked_backspace: bool,
}

pub fn get_user_inputs(ui: &egui::Ui) -> UserInputs
{
    let mut user_inputs = UserInputs::default();

    user_inputs.mouse_position = ui.input(|i| i.pointer.latest_pos().unwrap_or_default() );
    user_inputs.clicked_primary_mouse_button = ui.input(|i| i.pointer.primary_clicked() );
    user_inputs.holding_primary_mouse_button = ui.input(|i| i.pointer.primary_down() );
    user_inputs.released_primary_mouse_button = ui.input(|i| i.pointer.primary_released() );
    user_inputs.clicked_secondary_mouse_button = ui.input(|i| i.pointer.secondary_clicked() );
    user_inputs.holding_secondary_mouse_button = ui.input(|i| i.pointer.secondary_down() );
    user_inputs.released_secondary_mouse_button= ui.input(|i| i.pointer.secondary_released() );
    user_inputs.clicked_esp = ui.input(|i| { i.key_pressed(egui::Key::Escape) });
    user_inputs.holding_ctrl = ui.input(|i| { i.modifiers.ctrl });
    user_inputs.holding_shift = ui.input(|i| { i.modifiers.shift });
    user_inputs.holding_alt = ui.input(|i| { i.modifiers.alt });
    user_inputs.clicked_a = ui.input(|i| { i.key_pressed(egui::Key::A) });
    user_inputs.clicked_s = ui.input(|i| { i.key_pressed(egui::Key::S) });
    user_inputs.clicked_d = ui.input(|i| { i.key_pressed(egui::Key::D) });
    user_inputs.clicked_enter = ui.input(|i| { i.key_pressed(egui::Key::Enter) });
    user_inputs.clicked_up_arrow =  ui.input(|i| { i.key_pressed(egui::Key::ArrowUp) });
    user_inputs.clicked_down_arrow =  ui.input(|i| { i.key_pressed(egui::Key::ArrowDown) });
    user_inputs.clicked_delete =  ui.input(|i| { i.key_pressed(egui::Key::Delete) });
    user_inputs.clicked_backspace =  ui.input(|i| { i.key_pressed(egui::Key::Backspace) });

    user_inputs
}
