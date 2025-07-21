use crate::EmpowerData;

#[derive(Default, Clone)]
pub struct NumberNodeState
{
    pub data_conversion: EmpowerData, // @TODO, find a better name
}

impl NumberNodeState
{
    pub fn new() -> Self
    {
        Self 
        {  
            data_conversion: EmpowerData::Undefined( String::new() ),
        }
    }
}