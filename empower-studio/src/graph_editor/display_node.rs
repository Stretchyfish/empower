use crate::graph_editor::display_node::display_node_kind::DisplayState;

pub mod display_node_kind;

#[derive(Clone)]
pub struct DisplayNode
{
    pub title: &'static str,
    pub position: egui::Pos2,
    pub size: egui::Vec2,
    pub display_state: DisplayState,
}

impl DisplayNode
{
    pub fn new(title: &'static str, position: egui::Pos2, size: egui::Vec2, display_state: DisplayState) -> Self
    {
        Self
        {
            title,
            position,
            size, 
            display_state,
        }
    }
}




