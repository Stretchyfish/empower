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

#[derive(Clone, Serialize, Deserialize)]
pub struct Port
{
    pub key: NodeGraphKey,
    pub node_key: NodeGraphKey,
    pub direction: PortDirection,
    pub kind: PortKind,
    pub name: String,
    pub compatability: Vec<Value>,
    pub edit: PortEdit,
    pub value: Option<Value>,
}

impl Port
{
    pub fn new(key: NodeGraphKey, node_key: NodeGraphKey, port_definition: PortDefinition) -> Self
    {
        let value = if port_definition.compatability.is_empty() { None } else { Some( port_definition.compatability[0].clone() ) }; 

        let edit = match &value
        {
            None => PortEdit::None,
            Some( value_type ) => match value_type
            {
                Value::Integer( int ) => PortEdit::Text( int.to_string() ),
                Value::Float( float ) => PortEdit::Text(float.to_string()),
                Value::Bool( boolean ) => PortEdit::CheckBox( *boolean ),
            },
        };

        let mut port = Self
        {
            key,
            node_key,
            direction: port_definition.direction,
            kind: port_definition.kind,
            name: port_definition.name,
            compatability: port_definition.compatability,
            edit, 
            value: None,
        };

        port.check_if_parseble(); // @TODO, rewrite this

        port
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
                let value = if self.value.is_some()
                {
                    self.value.as_ref().unwrap()
                }
                else
                {
                    &self.compatability[0] // This should never be false
                };
                
                match value 
                {
                    Value::Integer(_) => egui::Color32::YELLOW,
                    Value::Float(_) => egui::Color32::BLUE,
                    Value::Bool(_) => egui::Color32::PURPLE,
                }
            }
        }
    }

    pub fn check_if_parseble(&mut self) // @TODO, change name
    {
        self.value = self.edit.convert_to_value(&self.compatability);
    }
}

