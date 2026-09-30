use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::{assets::AssetId, node_graph::port::PortDefinition};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum ReadFileState
{
    Asset( Option<AssetId> ),
    GlobalPath ( Option<PathBuf> ),
    RelativePath( Option<PathBuf> )
}

impl ReadFileState
{
    pub fn to_string(&self) -> String
    {
        match self
        {
            ReadFileState::Asset(_) => String::from("asset"),
            ReadFileState::GlobalPath(_) => String::from("global path"),
            ReadFileState::RelativePath(_) => String::from("relative path path"),
        }
    }

    pub fn get_input_port_definitions(&self) -> Vec<PortDefinition>
    {
        match self
        {
            ReadFileState::Asset(_) => vec![
                PortDefinition::new_input_execution_port(""),
                // Here the state should be able to select an asset
            ],
            ReadFileState::GlobalPath(_) => vec![
                PortDefinition::new_input_execution_port(""),
                
            ],
            ReadFileState::RelativePath(_) => vec![
                PortDefinition::new_input_execution_port(""),
                
            ],
        }
    }
}
