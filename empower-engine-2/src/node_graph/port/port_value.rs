// use std::convert::From;
use std::mem::discriminant;
use std::fmt;

#[derive(Default, Clone, Debug)]
pub enum PortValue
{
    Trigger,
    Integer(i32),
    Float(f32),
    Text(String),
    Bool(bool),
    Undefined(String),
    #[default] None,
}

impl PortValue
{
    pub fn is_same_type_as(&self, value: &PortValue) -> bool
    {
        discriminant(self) == discriminant(value)
    }
}

impl fmt::Display for PortValue
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        match *self
        {
            PortValue::Trigger => write!(f, "trigger"),
            PortValue::Integer(value) => write!(f, "{}", value),
            PortValue::Float(value) => write!(f, "{}", value),
            PortValue::Text(ref value) => write!(f, "{}", value),
            PortValue::Bool(value) => write!(f, "{}", value), 
            PortValue::Undefined(ref value) => write!(f, "{}", value),
            PortValue::None => write!(f, "none"),
        }
    }
}

// impl From<i32> for PortValue
// {
//     fn from(value: i32) -> Self 
//     {
//         PortValue::Integer(value)
//     }
// }

// impl From<f32> for PortValue
// {
//     fn from(value: f32) -> Self 
//     {
//         PortValue::Float(value)
//     }
// }
