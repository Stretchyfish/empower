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
    pub value: Option<Value>,
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
                Value::Integer( integer ) => PortEdit::Interger( integer.to_string() ),
                Value::Float( float ) => PortEdit::Float( float.to_string() ),
            },
        };
        
        Self
        {
            node_key,
            direction: port_definition.direction,
            kind: port_definition.kind,
            name: port_definition.name,
            value,
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
                match self.value.as_ref().unwrap() // This should never be false
                {
                    Value::Integer(_) => egui::Color32::YELLOW,
                    Value::Float(_) => egui::Color32::BLUE,
                }
            }
        }
    }

    pub fn attempt_to_parse_edit(&mut self)
    {
        match &self.edit
        {
            PortEdit::None => {},
            PortEdit::Interger( integer_string ) =>
            {
                let parsed_integer = integer_string.parse::<i32>();

                if parsed_integer.is_err()
                {
                    self.parseble = false;
                    return;
                }

                self.value = Some(Value::Integer( parsed_integer.unwrap() ));
            },
            PortEdit::Float(_) =>
            {
                todo!();
            },
        }

        self.parseble = true;
    }
}
