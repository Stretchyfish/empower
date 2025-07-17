use std::fmt::{self, write};
use std::mem::discriminant;

#[derive(Default, Clone)]
pub enum EmpowerData
{
    Trigger,
    Integer(i32),
    Float(f32),
    Undefined(String),
    #[default] Unknown
}

impl EmpowerData
{
    pub fn get_type(&self) -> String // @TODO, change name to get_type_string?
    {
        match *self
        {
            EmpowerData::Trigger => String::from("trigger"),
            EmpowerData::Integer(_) => String::from("int"),
            EmpowerData::Float(_) => String::from("float"),
            EmpowerData::Undefined(_) => String::from("undefined"),
            _ => String::from("unknown"),
        }
    }
}

impl fmt::Debug for EmpowerData
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result // @TODO, consider if this should be different long term
    {
        match *self
        {
            EmpowerData::Trigger => write!(f, "trigger"),
            EmpowerData::Integer(value) => write!(f, "{}", "int"),
            EmpowerData::Float(value) => write!(f, "{}", "float"),
            EmpowerData::Undefined(ref value) => write!(f, "{}", "undefined"),
            EmpowerData::Unknown => write!(f, "unknown"),
        }
    }
} 

impl fmt::Display for EmpowerData
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        match *self
        {
            EmpowerData::Trigger => write!(f, "trigger"),
            EmpowerData::Integer(value) => write!(f, "{}", value),
            EmpowerData::Float(value) => write!(f, "{}", value),
            EmpowerData::Undefined(ref value) => write!(f, "{}", value.clone()),
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

