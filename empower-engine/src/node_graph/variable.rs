use std::{collections::HashMap, sync::{Arc, Mutex}};

use crate::PortValue;

pub type Variables = HashMap<String, Arc<Mutex<Variable>>>;

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
