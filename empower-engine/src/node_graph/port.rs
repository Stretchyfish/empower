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
}
