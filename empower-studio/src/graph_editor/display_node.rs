use empower_engine::NodeKind;

pub mod display_node_kind;
pub mod display_node_registry;

#[derive(Default, Clone)]
pub struct DisplayNode
{
    pub title: String,
    pub position: egui::Pos2,
    pub size: egui::Vec2,
}

impl DisplayNode
{
    pub fn new(title: String,position: egui::Pos2, size: egui::Vec2) -> Self
    {
        Self
        {
            title,
            position,
            size, 
        }
    }
}

