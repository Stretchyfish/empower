use crate::compiler::InstructionAddress;

#[derive(Clone)]
pub struct ExecutorOutput
{
    pub instructions_executed: Vec<InstructionAddress>,
    pub outputs: Vec<String>,
}

impl ExecutorOutput
{
    pub fn new() -> Self
    {
        Self
        {
            instructions_executed: Vec::new(),
            outputs: Vec::new(),
        }
    }
}
