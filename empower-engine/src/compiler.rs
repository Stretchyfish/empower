use crate::project::Project;
use crate::value::Value;

mod debugger;
use debugger::DebugSettings;

mod instructions;
pub use instructions::Instruction;

pub fn debug_compile(project: &Project, debug_settings: &DebugSettings)
{
    
}

pub fn release_compile(project: &Project)
{
    
}
