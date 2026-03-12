use egui;

pub fn get_graph_viewport_user_inputs(ui: &mut egui::Ui) -> GraphViewportUserInputs
{
    let ui_rect = ui.max_rect();

    let mouse_is_inside_viewport = ui.ui_contains_pointer();

    let mouse_position = ui.input(|i| i.pointer.hover_pos()).unwrap_or(egui::Pos2 {x: 0.0, y: 0.0});
    let mut left_clicked = false;
    let mut released_left_click = false;
    let mut left_is_down = false;
    let mut right_clicked = false;
    let mut left_shift_is_down = false;
    let mut clicked_backspace = false;

    if ui_rect.contains(mouse_position) == true
    {
        left_clicked = ui.input(|i| i.pointer.primary_clicked());    
        left_is_down = ui.input(|i| i.pointer.primary_down());
        released_left_click = ui.input(|i| i.pointer.primary_released());
        right_clicked = ui.input(|i| i.pointer.secondary_clicked());
        left_shift_is_down = ui.input(|i| i.modifiers.shift);
        clicked_backspace = ui.input(|i| i.key_pressed(egui::Key::Backspace));
    }

    GraphViewportUserInputs
    {
        mouse_is_inside_viewport,
        mouse_position,
        left_clicked, 
        released_left_click,
        left_is_down,
        right_clicked,
        left_shift_is_down,
        clicked_backspace,
    }
}

#[derive(Default)]
pub struct GraphViewportUserInputs
{
    pub mouse_is_inside_viewport: bool,
    pub mouse_position: egui::Pos2,
    pub left_clicked: bool,
    pub left_is_down: bool,
    pub right_clicked: bool,
    pub released_left_click: bool,
    pub left_shift_is_down: bool,
    pub clicked_backspace: bool,
}
