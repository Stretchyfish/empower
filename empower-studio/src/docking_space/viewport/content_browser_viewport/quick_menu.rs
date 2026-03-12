#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct QuickMenu
{
    show: bool,
    mouse_position_when_quick_menu_was_activated: egui::Pos2,
}

