use serde::{Deserialize, Serialize};

use crate::{assets::AssetId, node_graph::port::PortDefinition, value::Value};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum LoopMode
{
    Forever,
    Range,
}

impl LoopMode
{
    pub fn to_string(&self) -> String
    {
        match self
        {
            LoopMode::Forever => String::from("forever"),
            LoopMode::Range => String::from("ranged"),
        }
    }

    pub fn get_input_port_definitions(&self) -> Vec<PortDefinition>
    {
        match self
        {
            LoopMode::Forever =>
            {
                vec![
                        PortDefinition::new_input_execution_port(),
                ]
            },
            LoopMode::Range =>
            {
                vec![
                        PortDefinition::new_input_execution_port(),
                        PortDefinition::new_input_data_port("start".to_string(), vec![ Value::Integer( 0 ) ] ),
                        PortDefinition::new_input_data_port("interval".to_string(), vec![ Value::Integer( 1 ) ] ),
                        PortDefinition::new_input_data_port("stop".to_string(), vec![ Value::Integer( 10 ) ] ),
                ]
            },
        }
    }

    pub fn get_output_port_definitions(&self) -> Vec<PortDefinition>
    {
        match self
        {
            LoopMode::Forever =>
            {
                vec![
                    PortDefinition::new_output_execution_port(),
                ]
            },
            LoopMode::Range =>
            {
                vec![
                    PortDefinition::new_output_execution_port(),
                    PortDefinition::new_output_data_port("condition".to_string(), vec![ Value::Bool( false ) ] ),
                    PortDefinition::new_output_data_port("value".to_string(), vec![ Value::Integer( 0 ) ] ),
                ]
            },
        }
    }
}
