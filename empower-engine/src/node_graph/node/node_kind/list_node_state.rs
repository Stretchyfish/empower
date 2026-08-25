use serde::{Deserialize, Serialize};

use crate::{node_graph::port::PortDefinition, utility::alphabet_counter::AlphabetCounter, value::Value};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct ListState
{
    pub value_type: Value,
    pub size: usize,
}

impl ListState
{
    pub fn new() -> Self
    {
        Self
        {
            value_type: Value::Integer(0),
            size: 2,
        }
    }

    pub fn get_input_port_definitions(&self) -> Vec<PortDefinition>
    {
        let mut inputs = Vec::with_capacity(self.size);

        let base_value = &self.value_type;

        let mut alphabet_counter = AlphabetCounter::new();

        for _ in 0..self.size
        {
            let letter = alphabet_counter.next_letter().to_string();
            inputs.push( PortDefinition::new_input_data_port(letter, vec![ base_value.clone() ]) );
        }
        
        inputs
    }

    pub fn get_output_port_definitions(&self) -> Vec<PortDefinition>
    {
        vec![
            PortDefinition::new_output_data_port("".to_string(), vec![ Value::List( Vec::new() ) ])
        ]
    }
}

