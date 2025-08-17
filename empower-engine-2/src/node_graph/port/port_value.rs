// use std::convert::From;
use std::mem::discriminant;

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
