use empower_engine::EmpowerKey;
use egui;

use super::PortSearcher;

pub struct NewGraphViewportState
{
    pub selected_nodes: Vec<EmpowerKey>,
    pub mouse_scene_position_last_frame: egui::Pos2,
    pub port_search: Option<PortSearcher>,
}

impl NewGraphViewportState
{
    pub fn new() -> Self
    {
        Self
        {
            selected_nodes: Vec::new(),
            mouse_scene_position_last_frame: egui::Pos2 { x: 0.0, y: 0.0 },
            port_search: None,
        }
    }
}
