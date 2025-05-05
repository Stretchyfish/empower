use empower_engine::EmpowerKey;
use egui;
use crate::viewports::graph_viewport::panels;
use crate::interactions;
use super::utils::PanZoom;
use super::utils::PortSearcher;

#[derive(Default)]
pub struct GraphViewportState
{
    pub title: String,
    pub selected_nodes: Vec<EmpowerKey>,
    pub node_select_rect: Option<egui::Rect>, // Consider improving the naming
    pub window_size: egui::Vec2,
    pub pan_zoom: PanZoom,
    pub dragging_background: bool,
    pub node_selection_panel_state: panels::NodeSelectionPanelState,
    pub port_search: Option<PortSearcher>,
}

impl GraphViewportState
{
    pub fn new(initial_title: String) -> Self
    {
        Self
        {
            title: initial_title,
            selected_nodes: Vec::new(),
            node_select_rect: Option::None,
            window_size: egui::Vec2 { x: 0.0, y: 0.0 },
            pan_zoom: PanZoom::new(),
            dragging_background: false,
            node_selection_panel_state: panels::NodeSelectionPanelState::new(),
            port_search: Option::None,
        }
    }    

}
