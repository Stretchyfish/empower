use serde::{Deserialize, Serialize};

use crate::{node_graph::{node::SettingsKind, port::PortDefinition}, value::Value};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct LoopMode
{
    pub mode: String,
}

impl LoopMode
{
    pub fn forever() -> Self
    {
        Self { mode: "forever".to_string() }
    }

    pub fn ranged() -> Self
    {
        Self { mode: "ranged".to_string() }
    }

    pub fn settings(&mut self) -> Vec<SettingsKind<'_>>
    {
        vec![SettingsKind::Choice {
            label: "mode",
            value: &mut self.mode,
            options: vec!["forever", "ranged"],
        }]
    }

    pub fn get_size(&self) -> egui::Vec2
    {
        match self.mode.as_str()
        {
            "ranged" => egui::vec2(480.0, 400.0),
            _ => egui::vec2(300.0, 220.0),
        }
    }

    pub fn get_input_port_definitions(&self) -> Vec<PortDefinition>
    {
        match self.mode.as_str()
        {
            "ranged" =>
            {
                vec![
                        PortDefinition::new_input_execution_port(""),
                        PortDefinition::new_input_data_port("start".to_string(), vec![ Value::Integer( 0 ) ] ),
                        PortDefinition::new_input_data_port("interval".to_string(), vec![ Value::Integer( 1 ) ] ),
                        PortDefinition::new_input_data_port("stop".to_string(), vec![ Value::Integer( 10 ) ] ),
                ]
            },
            _ =>
            {
                vec![
                        PortDefinition::new_input_execution_port(""),
                ]
            },
        }
    }

    pub fn get_output_port_definitions(&self) -> Vec<PortDefinition>
    {
        match self.mode.as_str()
        {
            "ranged" =>
            {
                vec![
                    PortDefinition::new_output_execution_port(""),
                    PortDefinition::new_output_data_port("condition".to_string(), vec![ Value::Bool( false ) ] ),
                    PortDefinition::new_output_data_port("value".to_string(), vec![ Value::Integer( 0 ) ] ),
                ]
            },
            _ =>
            {
                vec![
                    PortDefinition::new_output_execution_port(""),
                ]
            },
        }
    }
}
