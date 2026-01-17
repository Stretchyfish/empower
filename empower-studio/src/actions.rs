use empower_engine::NodeGraphKey;
use crate::graph_editor::DisplayValue;

pub enum Action
{
    CreateNode { name: &'static str, position: egui::Pos2 },
    RefreshNodeStructure { node_key: NodeGraphKey },
    DeleteNode { node_key: NodeGraphKey },
    CopySelectedNodes,
    ToggleNodeSelection { node_key: NodeGraphKey },
    AddNodesToSelectedNodes { node_keys: Vec<NodeGraphKey> }, 
    ClearAllNodesFromSelectedNodes,
    MoveSelectedNodes { canvas_delta_position: egui::Vec2 },
    ClickedInputPort { port_key: NodeGraphKey },
    SetInputPortValue { port_key: NodeGraphKey, display_value: DisplayValue},
    ClickedOutputPort { port_key: NodeGraphKey },
    StopPortSearch,
    CreateViewport { name: &'static str },
    ToggleDebugWindow,
    ToggleExecutionHisotryWindow,
    StartNodeGraphExecution,
    StartNodeGraphExecutionFromEntry { node_key: NodeGraphKey },
    StopNodeGraphExecution,
}
