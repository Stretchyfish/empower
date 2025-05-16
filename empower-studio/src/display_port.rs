use empower_engine::EmpowerKey;

pub struct DisplayPort
{
    pub key: EmpowerKey,
    pub node_key: EmpowerKey,
    pub relative_position: egui::Vec2,
    pub value: String,
}
