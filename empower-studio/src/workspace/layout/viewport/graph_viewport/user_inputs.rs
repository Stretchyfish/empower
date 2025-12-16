use egui;

pub fn get_graph_viewport_user_inputs(ui: &mut egui::Ui) -> GraphViewportUserInputs
{
    let ui_rect = ui.max_rect();

    let mouse_position = ui.input(|i| i.pointer.hover_pos()).unwrap_or(egui::Pos2 {x: 0.0, y: 0.0});
    let mut left_clicked = false;
    let mut left_is_down = false;
    let mut right_clicked = false;
    let mut left_shift_is_down = false;

    if ui_rect.contains(mouse_position) == true
    {
        left_clicked = ui.input(|i| i.pointer.primary_clicked());    
        left_is_down = ui.input(|i| i.pointer.primary_down());
        right_clicked = ui.input(|i| i.pointer.secondary_clicked());
        left_shift_is_down = ui.input(|i| i.modifiers.shift);
    }

    GraphViewportUserInputs
    {
        mouse_position: mouse_position,
        left_clicked: left_clicked, 
        left_is_down: left_is_down,
        right_clicked: right_clicked,
        left_shift_is_down: left_shift_is_down,
    }
}

#[derive(Default)]
pub struct GraphViewportUserInputs
{
    pub mouse_position: egui::Pos2,
    pub left_clicked: bool,
    pub left_is_down: bool,
    pub right_clicked: bool,
    pub left_shift_is_down: bool,
}