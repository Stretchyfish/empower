use empower_node_graph::EmpowerKey;

#[derive(Default, Clone)]
pub struct DisplayPort
{
    pub node_key: EmpowerKey,
    pub relative_position: egui::Vec2,
    pub text: String,
    pub value_representation: DisplayPortValueRepresentation, // @TODO, consider changing this name?
    pub value_representation_valid: bool,
}

#[derive(Default, Clone)]
pub enum DisplayPortValueRepresentation // @TODO, find a better name?
{
    Text(String),
    Checkbox(bool),
    #[default] None, //@TODO, consider if this should be named something else?
}

