use empower_engine::NodeGraphKey;

pub enum Action
{
    CreateNode { name: &'static str, position: egui::Pos2 },
    ToggleNodeSelection { node_key: NodeGraphKey },
    MoveSelectedNodes { canvas_delta_position: egui::Vec2 },
    CreateViewport { name: &'static str },
    ToggleDebugWindow,
    StartNodeGraphExecution,
    StopNodeGraphExecution,
    None,
}
