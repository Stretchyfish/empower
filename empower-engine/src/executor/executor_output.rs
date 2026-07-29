use crate::{assets::AssetId, compiler::InstructionAddress};

#[derive(Clone)]
pub struct ExecuteUnit
{
    pub graph_id: AssetId,
    pub instruction_address: InstructionAddress,
}

#[derive(Clone)]
pub struct ExecutorOutput
{
    pub execute_units: Vec<ExecuteUnit>,
    pub outputs: Vec<String>,
}

impl ExecutorOutput
{
    pub fn new() -> Self
    {
        Self
        {
            execute_units: Vec::new(),
            outputs: Vec::new(),
        }
    }
}
