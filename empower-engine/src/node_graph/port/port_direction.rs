use serde::{Deserialize, Serialize};

#[derive(PartialEq, PartialOrd, Clone, Copy, Serialize, Deserialize)]
pub enum PortDirection
{
    Input,
    Output
}

