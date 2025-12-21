use empower_engine::NodeGraphKey;

pub enum Action
{
    CreateNode { name: &'static str, position: egui::Pos2 },
    RemoveNode { node_key: NodeGraphKey },
    CopyNode { node_key: NodeGraphKey },
    ToggleNodeSelection { node_key: NodeGraphKey },
    AddNodeToSelectedNodes { node_key: NodeGraphKey },
    AddNodesToSelectedNodes { node_keys: Vec<NodeGraphKey> },
    RemoveAllNodesFromSelectedNodes,
    MoveSelectedNodes { canvas_delta_position: egui::Vec2 },
    ClickedInputPort { port_key: NodeGraphKey },
    ClickedOutputPort { port_key: NodeGraphKey },
    CreateViewport { name: &'static str },
    ToggleDebugWindow,
    StartNodeGraphExecution,
    StartNodeGraphExecutionFromEntry { node_key: NodeGraphKey },
    StopNodeGraphExecution,
    None,
}
