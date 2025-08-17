// use std::convert::From;

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
