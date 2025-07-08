use std::fmt;
use std::mem::discriminant;

#[derive(Default, Clone, Copy)]
pub enum EmpowerData
{
    Trigger,
    Integer(i32),
    #[default] Unknown
}

impl fmt::Display for EmpowerData
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        match *self
        {
            EmpowerData::Trigger => write!(f, "trigger"),
            EmpowerData::Integer(value) => write!(f, "{}", value),
            EmpowerData::Unknown => write!(f, "unknown"),
        }
    }
}

impl PartialEq for EmpowerData
{
    fn eq(&self, other: &Self) -> bool 
    {
        discriminant(self) == discriminant(other)
    }
}