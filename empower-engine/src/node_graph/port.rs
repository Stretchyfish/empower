use crate::node_graph::NodeGraphKey;
use crate::value::Value;
use serde::{Deserialize, Serialize};

mod port_definition;
pub use port_definition::PortDefinition;

mod port_kind;
pub use port_kind::PortKind;

mod port_direction;
pub use port_direction::PortDirection;

mod port_edit;
pub use port_edit::PortEdit;

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct Port
{
    pub key: NodeGraphKey,
    pub node_key: NodeGraphKey,
    pub direction: PortDirection,
    pub kind: PortKind,
    pub name: String,
    pub compatability: Vec<Value>,
    pub value: Option<Value>, // Is optional because of exec port
}

impl Port
{
    pub fn new(key: NodeGraphKey, node_key: NodeGraphKey, port_definition: PortDefinition) -> Self
    {
        let value = if port_definition.compatability.is_empty() { None } else { Some( port_definition.compatability[0].clone() ) };

        let port = Self
        {
            key,
            node_key,
            direction: port_definition.direction,
            kind: port_definition.kind,
            name: port_definition.name,
            compatability: port_definition.compatability,
            value: value,
        };

        port
    }

    pub fn to_port_definitions(&self) -> PortDefinition
    {
        PortDefinition
        {
            name: self.name.clone(),
            direction: self.direction,
            kind: self.kind,
            compatability: self.compatability.clone()
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
                .any(|from_possible_value| port.compatability.iter().any(|to_possible_value| *from_possible_value.type_string() == *to_possible_value.type_string() )),
            _ => false,
        }
    }
}

