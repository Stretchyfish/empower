use std::fmt::{self, write};
use std::mem::discriminant;

#[derive(Default, Clone)]
pub enum EmpowerData
{
    Trigger,
    Integer(i32),
    Float(f32),
    Text(String),
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
            EmpowerData::Text(_) => String::from("text"),
            EmpowerData::Undefined(_) => String::from("undefined"),
            EmpowerData::Unknown => String::from("Unknown"),
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
            EmpowerData::Integer(_) => write!(f, "{}", "int"),
            EmpowerData::Float(_) => write!(f, "{}", "float"),
            EmpowerData::Text(_) => write!(f, "{}", "text"),
            EmpowerData::Undefined(_) => write!(f, "{}", "undefined"),
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
            EmpowerData::Text(ref value) => write!(f, "{}", value),
            EmpowerData::Undefined(ref value) => write!(f, "{}", value),
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

