// use crate::graph_editor::display_node::display_node_kind::DisplayNodeKind;

pub mod display_node_kind;
pub mod display_node_registry;

pub struct DisplayNode
{
    pub title: String,
    pub position: egui::Pos2,
    pub size: egui::Vec2,
    // pub kind: Box<dyn DisplayNodeKind>,
}

impl DisplayNode
{
    // To be added back later
    // pub fn new(title: String, position: egui::Pos2, size: egui::Vec2, kind: Box<dyn DisplayNodeKind>) -> Self
    pub fn new(title: String, position: egui::Pos2, size: egui::Vec2) -> Self
    {
        Self
        {
            title,
            position,
            size, 
            // kind,
        }
    }
}




