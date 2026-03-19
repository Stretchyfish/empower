use std::collections::HashMap;

use crate::PortValue;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Variable
{
    pub name: String,
    pub values: HashMap<String, PortValue>,
}

impl Variable
{
    pub fn new() -> Self
    {
        Self
        {
            name: String::new(),
            values: HashMap::new(),
        }
    }
}
