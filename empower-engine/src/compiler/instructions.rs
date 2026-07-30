use crate::{assets::AssetId, compiler::RegisterAddress, value::Value};

pub type InstructionSet = Vec<Instruction>;

pub type InstructionAddress = usize;

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum Instruction
{
    SetConst( RegisterAddress, Value ),
    Copy ( RegisterAddress, RegisterAddress),
    Add ( RegisterAddress, RegisterAddress, RegisterAddress ),
    Compare( RegisterAddress, RegisterAddress, RegisterAddress ),
    CreateList ( Vec<RegisterAddress>, RegisterAddress ), // @TODO, find a better name?
    Jump(InstructionAddress),
    JumpIfFalse(InstructionAddress, RegisterAddress),
    JumpIfTrue(InstructionAddress, RegisterAddress),
    Print(RegisterAddress),
    Return,
    Wait(RegisterAddress),
    CallGraph ( AssetId ),
    ShowImage ( RegisterAddress ),
    ShowMathGraph ( RegisterAddress ),
    Fork( InstructionAddress ),
    Join,
}

impl Instruction
{
    pub fn to_string(&self) -> String
    {
        match self
        {
            Instruction::SetConst( register_address, value ) => format!("SetConst ( r_addr: {}, value: {})", register_address, value.to_string()),
            Instruction::Copy( from_register_address, to_register_address) => format!("Copy ( r_addr: {}, r_addr: {})", from_register_address, to_register_address),
            Instruction::CreateList ( from_addresses_of_elements, to_register_address ) => format!("CreateList ( v_r_addr {:?}, r_addr: {})", from_addresses_of_elements, to_register_address),
            Instruction::Jump( instruction_address ) => format!("Jump ( i_addr: {})", instruction_address),
            Instruction::JumpIfFalse( instruction_address, register_address ) => format!("JumpIfFalse( i_addr: {}, i_addr: {})", instruction_address, register_address ),
            Instruction::Print( register_address ) => format!("Print ( r_addr: {})", register_address),
            Instruction::Return => "Return".to_string(),
            Instruction::Wait( register_address ) => format!("Wait ( r_addr: {})", register_address),
            Instruction::CallGraph( node_graph_id ) => format!("CallGraph ( asse_id: {})", node_graph_id),
            Instruction::ShowImage( register_address ) => format!("ShowImage( r_addr: {})", register_address ),
            Instruction::ShowMathGraph( register_address ) => format!("ShowMathGraph ( r_addr: {})", register_address),
            Instruction::Fork( instruction_address ) => format!("Fork( i_addr: {})", instruction_address),
            Instruction::Join => String::from("Join"),
            Instruction::Add( result_register_address, register_address_1, register_address_2 ) => format!("Add ( r_addr: {}, r_addr: {}, r_addr: {})", result_register_address, register_address_1, register_address_2),
            Instruction::Compare( result_register_address, register_address_1, register_address_2 ) => format!("Compare ( r_addr: {}, r_addr: {}, r_addr: {})", result_register_address, register_address_1, register_address_2),
            Instruction::JumpIfTrue( instruction_address, register_address ) => format!("JumpIfTrue( i_addr: {}, i_addr: {})", instruction_address, register_address ),
        }
    }
}
