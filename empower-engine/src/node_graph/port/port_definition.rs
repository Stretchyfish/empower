use crate::value::Value;

use super::PortDirection;
use super::PortKind;

pub struct PortDefinition
{
    pub name: &'static str,
    pub direction: PortDirection,
    pub kind: PortKind,
    pub compatability: Vec<Value>,
}

impl PortDefinition
{
    pub fn new_input_execution_port() -> Self
    {
        Self
        {
            name: "",
            direction: PortDirection::Input,
            kind: PortKind::Execution,
            compatability: Vec::new(),
        }
    }

    pub fn new_output_execution_port() -> Self
    {
        Self
        {
            name: "",
            direction: PortDirection::Output,
            kind: PortKind::Execution,
            compatability: Vec::new(),
        }
    }

    pub fn new_input_data_port(name: &'static str, compatability: Vec<Value>) -> Self
    {
        Self
        {
            name,
            direction: PortDirection::Input,
            kind: PortKind::Data,
            compatability,
        }
    }

    pub fn new_output_data_port(name: &'static str, compatability: Vec<Value>) -> Self
    {
        Self
        {
            name,
            direction: PortDirection::Output,
            kind: PortKind::Data,
            compatability,
        }
    }
}
