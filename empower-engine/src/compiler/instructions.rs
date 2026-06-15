use crate::compiler::RegisterAddress;

pub type InstructionSet = Vec<Instruction>;

pub type InstructionAddress = usize;

pub enum Instruction
{
    Jump(InstructionAddress),
    Print(RegisterAddress),
}

impl Instruction
{
    pub fn to_string(&self) -> String
    {
        match self
        {
            Instruction::Jump( instruction_address ) => format!("Jump ({})", instruction_address),
            Instruction::Print( register_address ) => format!("Print ({})", register_address),
        }
    }
}
