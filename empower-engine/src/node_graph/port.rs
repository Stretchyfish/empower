use crate::node_graph::NodeGraphKey;
use crate::value::Value;

mod port_definition;
pub use port_definition::PortDefinition;

mod port_kind;
pub use port_kind::PortKind;

mod port_direction;
pub use port_direction::PortDirection;

pub struct Port
{
    pub node_key: NodeGraphKey,
    pub direction: PortDirection,
    pub kind: PortKind,
    pub name: &'static str,
    pub value: Option<Value>,
    pub compatability: Vec<Value>,
}

impl Port
{
    pub fn new(node_key: NodeGraphKey, port_definition: PortDefinition) -> Self
    {
        Self
        {
            node_key,
            direction: port_definition.direction,
            kind: port_definition.kind,
            name: port_definition.name,
            value: if port_definition.compatability.is_empty() { None } else { Some( port_definition.compatability[0].clone() ) },
            compatability: port_definition.compatability,
        }
    }

    pub fn compatible_with(&self, port: &Port) -> bool
    {
        match (self.kind, port.kind)
        {
            (PortKind::Execution, PortKind::Execution) =>
                return true,
            (PortKind::Data, PortKind::Data) =>
                self.compatability.iter()
                .any(|from_possible_value| port.compatability.iter().any(|to_possible_value| *from_possible_value == *to_possible_value )),
            _ => false,
        }
    }

    pub fn color(&self) -> egui::Color32
    {
        match self.kind
        {
            PortKind::Execution => egui::Color32::WHITE,
            PortKind::Data =>
            {
                match self.value.as_ref().unwrap() // This should never be false
                {
                    Value::Integer(_) => egui::Color32::YELLOW,
                    Value::Float(_) => egui::Color32::BLUE,
                }
            }
        }
    }
}
