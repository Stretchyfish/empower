use crate::node_graph::NodeGraphKey;
use crate::value::Value;

mod port_definition;
pub use port_definition::PortDefinition;

mod port_kind;
pub use port_kind::PortKind;

mod port_direction;
pub use port_direction::PortDirection;

mod port_edit;
pub use port_edit::PortEdit;

pub struct Port
{
    pub node_key: NodeGraphKey,
    pub direction: PortDirection,
    pub kind: PortKind,
    pub name: &'static str,
    pub compatability: Vec<Value>,
    pub edit: PortEdit,
    pub parseble: bool,
}

impl Port
{
    pub fn new(node_key: NodeGraphKey, port_definition: PortDefinition) -> Self
    {
        let value = if port_definition.compatability.is_empty() { None } else { Some( port_definition.compatability[0].clone() ) }; 

        let edit = match &value
        {
            None => PortEdit::None,
            Some( value_type ) => match value_type
            {
                Value::Integer => PortEdit::Interger( "0".to_string() ),
                Value::Float => PortEdit::Float( "0.0".to_string() ),
            },
        };
        
        Self
        {
            node_key,
            direction: port_definition.direction,
            kind: port_definition.kind,
            name: port_definition.name,
            compatability: port_definition.compatability,
            edit, 
            parseble: true,
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
                match self.compatability[0] // This should never be false
                {
                    Value::Integer => egui::Color32::YELLOW,
                    Value::Float => egui::Color32::BLUE,
                }
            }
        }
    }

    pub fn able_to_parse_edit(&mut self)
    {
        self.parseble = match &self.edit
        {
            PortEdit::None => true,
            PortEdit::Interger( integer_string ) => integer_string.parse::<i32>().is_ok(),
            PortEdit::Float(_) =>
            {
                todo!();
            },
        }
    }
}
