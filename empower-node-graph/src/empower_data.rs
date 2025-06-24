use std::fmt;

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum EmpowerData
{
    Integer(i32),
    #[default] Unknown
}

impl fmt::Display for EmpowerData
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        match *self
        {
            EmpowerData::Integer(value) => write!(f, "{}", value),
            EmpowerData::Unknown => write!(f, "unknown"),
        }
    }
}
