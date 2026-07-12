use crate::{assets::AssetId, compiler::RegisterAddress, value::Value};

pub type InstructionSet = Vec<Instruction>;

pub type InstructionAddress = usize;

#[derive(Clone)]
pub enum Instruction
{
    SetConst( RegisterAddress, Value ),
    Copy ( RegisterAddress, RegisterAddress),
    Jump(InstructionAddress),
    JumpIfFalse(InstructionAddress, RegisterAddress),
    Print(RegisterAddress),
    Return,
    Wait(RegisterAddress),
    CallGraph ( AssetId ),
    ShowImage ( AssetId ),
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
            Instruction::JumpIfFalse( instruction_address, register_address ) => format!("JumpIfFalse({}, {})", instruction_address, register_address ),
            Instruction::Print( register_address ) => format!("Print ({})", register_address),
            Instruction::Return => "Return".to_string(),
            Instruction::Wait( register_address ) => format!("Wait ({})", register_address),
            Instruction::CallGraph( node_graph_id ) => format!("CallGraph ({})", node_graph_id),
            Instruction::ShowImage( image_id ) => format!("ShowImage({})", image_id),
        }
    }
}
