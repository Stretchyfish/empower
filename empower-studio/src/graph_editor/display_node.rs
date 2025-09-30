pub mod display_node_kind;
pub mod display_node_registry;

pub struct DisplayNode
{
    pub title: &'static str,
    pub position: egui::Pos2,
    pub size: egui::Vec2,
}

impl DisplayNode
{
    pub fn new(title: &'static str, position: egui::Pos2, size: egui::Vec2) -> Self
    {
        Self
        {
            title,
            position,
            size, 
        }
    }
}




