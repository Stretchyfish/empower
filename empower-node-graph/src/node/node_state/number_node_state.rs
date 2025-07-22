use crate::EmpowerData;
use std::fmt;

#[derive(Default, Clone)]
pub struct NumberNodeState
{
    pub current_convertion_approach: ConvertionApproach,
}

impl NumberNodeState
{
    pub fn new() -> Self
    {
        Self 
        {  
            current_convertion_approach: ConvertionApproach::Automatic,
        }
    }
}

#[derive(Default, Clone, PartialEq, Eq)]
pub enum ConvertionApproach // @TODO, find a better name
{
    #[default] Automatic,
    Float,
    Int,
}

impl fmt::Display for ConvertionApproach
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        match *self
        {
            ConvertionApproach::Automatic => write!(f, "Automatic"),
            ConvertionApproach::Int => write!(f, "int"),
            ConvertionApproach::Float => write!(f, "float"), 
        }
    }
}
