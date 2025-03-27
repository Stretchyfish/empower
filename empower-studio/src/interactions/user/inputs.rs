use egui;

#[derive(Default)]
pub struct UserInputs
{
    pub mouse_position: egui::Pos2,
    pub mouse_position_delta: egui::Vec2,
    pub mouse_is_in_ui: bool,
    pub left_clicked: bool,
    pub left_is_down: bool,
    pub right_clicked: bool,
    pub middle_is_down: bool,
    pub scroll_delta: f32,
    pub left_shift_is_down: bool,
}

pub fn detect_user_inputs(ui: &egui::Ui) -> UserInputs
{
    let ui_rect = ui.max_rect();

    let mouse_position = ui.input(|i| i.pointer.hover_pos()).unwrap_or(egui::Pos2 {x: 0.0, y: 0.0});
    let mouse_position_delta = ui.input(|i| i.pointer.delta() );

    let mut left_clicked = false;
    let mut left_is_down = false;
    let mut right_clicked = false;
    let mut middle_is_down = false;
    let mut scroll_delta= 0.0;
    let mut left_shift_is_down = false;

    let mut mouse_is_inside_rect = false;
    if ui_rect.contains(mouse_position) == true
    {
        mouse_is_inside_rect = true;
        left_clicked = ui.input(|i| i.pointer.primary_clicked());    
        left_is_down = ui.input(|i| i.pointer.primary_down());
        right_clicked = ui.input(|i| i.pointer.secondary_clicked());
        middle_is_down = ui.input(|i| i.pointer.middle_down());
        scroll_delta = ui.input(|i| i.smooth_scroll_delta.y);
        left_shift_is_down = ui.input(|i| i.modifiers.shift);
    }

    UserInputs
    {
        mouse_position: mouse_position,
        mouse_position_delta: mouse_position_delta,
        mouse_is_in_ui: mouse_is_inside_rect,
        left_clicked: left_clicked, 
        left_is_down: left_is_down,
        right_clicked: right_clicked,
        middle_is_down: middle_is_down,
        scroll_delta: scroll_delta,
        left_shift_is_down: left_shift_is_down,
    }
}
