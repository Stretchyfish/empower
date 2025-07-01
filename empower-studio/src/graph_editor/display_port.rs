use empower_node_graph::EmpowerKey;

#[derive(Default, Clone)]
pub struct DisplayPort
{
    pub node_key: EmpowerKey,
    pub relative_position: egui::Vec2,
    pub value: String,
}

