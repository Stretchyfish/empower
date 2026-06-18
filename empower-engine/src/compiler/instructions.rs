use crate::{compiler::RegisterAddress, value::Value};

pub type InstructionSet = Vec<Instruction>;

pub type InstructionAddress = usize;

#[derive(Clone)]
pub enum Instruction
{
    SetConst( RegisterAddress, Value ),
    Copy ( RegisterAddress, RegisterAddress),
    Jump(InstructionAddress),
    Print(RegisterAddress),
    Return,
}

impl Instruction
{
    pub fn to_string(&self) -> String
    {
        match self
        {
            Instruction::SetConst( register_address, value ) => format!("SetConst ({}, {})", register_address, value.to_string()),
            Instruction::Copy( from_register_address, to_register_address) => format!("Copy ({}, {})", from_register_address, to_register_address),
            Instruction::Jump( instruction_address ) => format!("Jump ({})", instruction_address),
            Instruction::Print( register_address ) => format!("Print ({})", register_address),
            Instruction::Return => "Return".to_string(),
        }
    }
}
