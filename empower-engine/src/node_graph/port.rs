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
    pub value: Option<Value>, // Is optional because of exec port
}

impl Port
{
    pub fn new(key: NodeGraphKey, node_key: NodeGraphKey, port_definition: PortDefinition) -> Self
    {
        let value = if port_definition.compatability.is_empty() { None } else { Some( port_definition.compatability[0].clone() ) };

        let edit = match &value
        {
            None => PortEdit::None,
            Some( actual_value ) => match actual_value
            {
                Value::Integer( int ) => PortEdit::Text( int.to_string() ),
                Value::Float( float ) => PortEdit::Text( float.to_string() ),
                Value::Bool( boolean ) => PortEdit::CheckBox( *boolean ),
                Value::Image( _ ) => PortEdit::None,
                Value::Point2d( x, y ) => PortEdit::TwoBox( x.to_string(), y.to_string() ),
                Value::List( _ ) => PortEdit::None,
            },
        };

        let port = Self
        {
            key,
            node_key,
            direction: port_definition.direction,
            kind: port_definition.kind,
            name: port_definition.name,
            compatability: port_definition.compatability,
            edit, 
            value: value,
        };

        // port.check_if_parseble(); // @TODO, rewrite this

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
                    Value::Image(_) => egui::Color32::GREEN,
                    Value::Point2d(_, _) => egui::Color32::ORANGE,
                    Value::List(_) => egui::Color32::PURPLE,
                }
            }
        }
    }

    pub fn check_if_parseble(&mut self) // @TODO, change name
    {
        self.value = self.edit.convert_to_value(&self.compatability);
    }
}

