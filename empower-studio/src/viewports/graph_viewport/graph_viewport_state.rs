use empower_engine::EmpowerKey;
use egui;

use super::node_selection_panel::NodeSelectionPanelState;

use super::PortSearcher;

pub struct GraphViewportState
{
    pub selected_nodes: Vec<EmpowerKey>,
    pub node_selection_panel: NodeSelectionPanelState,
    pub mouse_scene_position_last_frame: egui::Pos2,
    pub port_search: Option<PortSearcher>,
}

impl GraphViewportState
{
    pub fn new() -> Self
    {
        Self
        {
            selected_nodes: Vec::new(),
            node_selection_panel: NodeSelectionPanelState::new(),
            mouse_scene_position_last_frame: egui::Pos2 { x: 0.0, y: 0.0 },
            port_search: None,
        }
    }
}
