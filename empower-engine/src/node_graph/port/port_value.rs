// use std::convert::From;
use std::mem::discriminant;
use std::fmt;
use std::ops;

#[derive(Default, Clone, Debug, PartialEq)]
pub enum PortValue
{
    Trigger,
    Integer(i32),
    Float(f32),
    Text(String),
    Bool(bool),
    Vector(Vec<PortValue>),
    Undefined(String), // @TODO, consider removing undefined
    #[default] None,
}

impl PortValue
{
    pub fn is_same_type_as(&self, value: &PortValue) -> bool
    {
        discriminant(self) == discriminant(value)
    }

    pub fn as_desired_value(&self, value: &PortValue) -> Option<PortValue>
    {
        match (self, value)
        {
            (PortValue::Trigger, PortValue::Trigger) => todo!(),
            (PortValue::Trigger, PortValue::Integer(_)) => todo!(),
            (PortValue::Trigger, PortValue::Float(_)) => todo!(),
            (PortValue::Trigger, PortValue::Text(_)) => todo!(),
            (PortValue::Trigger, PortValue::Bool(_)) => todo!(),
            (PortValue::Trigger, PortValue::Undefined(_)) => todo!(),
            (PortValue::Trigger, PortValue::None) => todo!(),
            (PortValue::Integer(_), PortValue::Trigger) => todo!(),
            (PortValue::Integer(a), PortValue::Integer(_)) => Some( PortValue::Integer( a.clone() ) ),
            (PortValue::Integer(a), PortValue::Float(_)) => Some( PortValue::Float( a.clone() as f32 )),
            (PortValue::Integer(_), PortValue::Text(_)) => todo!(),
            (PortValue::Integer(_), PortValue::Bool(_)) => todo!(),
            (PortValue::Integer(_), PortValue::Undefined(_)) => todo!(),
            (PortValue::Integer(_), PortValue::None) => todo!(),
            (PortValue::Float(_), PortValue::Trigger) => todo!(),
            (PortValue::Float(_), PortValue::Integer(_)) => todo!(),
            (PortValue::Float(_), PortValue::Float(_)) => todo!(),
            (PortValue::Float(_), PortValue::Text(_)) => todo!(),
            (PortValue::Float(_), PortValue::Bool(_)) => todo!(),
            (PortValue::Float(_), PortValue::Undefined(_)) => todo!(),
            (PortValue::Float(_), PortValue::None) => todo!(),
            (PortValue::Text(_), PortValue::Trigger) => todo!(),
            (PortValue::Text(_), PortValue::Integer(_)) => todo!(),
            (PortValue::Text(_), PortValue::Float(_)) => todo!(),
            (PortValue::Text(_), PortValue::Text(_)) => todo!(),
            (PortValue::Text(_), PortValue::Bool(_)) => todo!(),
            (PortValue::Text(_), PortValue::Undefined(_)) => todo!(),
            (PortValue::Text(_), PortValue::None) => todo!(),
            (PortValue::Bool(_), PortValue::Trigger) => todo!(),
            (PortValue::Bool(_), PortValue::Integer(_)) => todo!(),
            (PortValue::Bool(_), PortValue::Float(_)) => todo!(),
            (PortValue::Bool(_), PortValue::Text(_)) => todo!(),
            (PortValue::Bool(_), PortValue::Bool(_)) => todo!(),
            (PortValue::Bool(_), PortValue::Undefined(_)) => todo!(),
            (PortValue::Bool(_), PortValue::None) => todo!(),
            (PortValue::Undefined(_), PortValue::Trigger) => todo!(),
            (PortValue::Undefined(_), PortValue::Integer(_)) => todo!(),
            (PortValue::Undefined(_), PortValue::Float(_)) => todo!(),
            (PortValue::Undefined(_), PortValue::Text(_)) => todo!(),
            (PortValue::Undefined(_), PortValue::Bool(_)) => todo!(),
            (PortValue::Undefined(_), PortValue::Undefined(_)) => todo!(),
            (PortValue::Undefined(_), PortValue::None) => todo!(),
            (PortValue::None, PortValue::Trigger) => todo!(),
            (PortValue::None, PortValue::Integer(_)) => todo!(),
            (PortValue::None, PortValue::Float(_)) => todo!(),
            (PortValue::None, PortValue::Text(_)) => todo!(),
            (PortValue::None, PortValue::Bool(_)) => todo!(),
            (PortValue::None, PortValue::Undefined(_)) => todo!(),
            (PortValue::None, PortValue::None) => todo!(),
            (PortValue::Trigger, PortValue::Vector(_)) => todo!(),
            (PortValue::Integer(_), PortValue::Vector(_)) => todo!(),
            (PortValue::Float(_), PortValue::Vector(_)) => todo!(),
            (PortValue::Text(_), PortValue::Vector(_)) => todo!(),
            (PortValue::Bool(_), PortValue::Vector(_)) => todo!(),
            (PortValue::Vector(_), PortValue::Trigger) => todo!(),
            (PortValue::Vector(_), PortValue::Integer(_)) => todo!(),
            (PortValue::Vector(_), PortValue::Float(_)) => todo!(),
            (PortValue::Vector(_), PortValue::Text(_)) => todo!(),
            (PortValue::Vector(_), PortValue::Bool(_)) => todo!(),
            (PortValue::Vector(_), PortValue::Vector(_)) => todo!(),
            (PortValue::Vector(_), PortValue::Undefined(_)) => todo!(),
            (PortValue::Vector(_), PortValue::None) => todo!(),
            (PortValue::Undefined(_), PortValue::Vector(_)) => todo!(),
            (PortValue::None, PortValue::Vector(_)) => todo!(),
        }
    }
}

impl fmt::Display for PortValue
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        match self
        {
            PortValue::Trigger => write!(f, "trigger"),
            PortValue::Integer(value) => write!(f, "{}", value),
            PortValue::Float(value) => write!(f, "{}", value),
            PortValue::Text(value) => write!(f, "{}", value),
            PortValue::Bool(value) => write!(f, "{}", value),
            PortValue::Undefined(value) => write!(f, "{}", value),
            PortValue::None => write!(f, "none"),
            PortValue::Vector(port_values) => write!(f, "{:?}", port_values)
        }
    }
}

impl ops::Add for PortValue
{
    type Output = PortValue;

    fn add(self, rhs: Self) -> Self::Output
    {
        match (self, rhs)
        {
            ( PortValue::Integer(a), PortValue::Integer(b) ) => PortValue::Integer( a + b),
            ( PortValue::Float(a), PortValue::Float(b) ) => PortValue::Float( a + b),
            ( PortValue::Float(a), PortValue::Integer(b) ) => PortValue::Float( a + b as f32),
            ( PortValue::Integer(a), PortValue::Float(b) ) => PortValue::Float( a as f32 + b),
            _ => panic!("Tried to do addition on two incompatible types"),
        }
    }
}

impl ops::Mul for PortValue
{
    type Output = PortValue;

    fn mul(self, rhs: Self) -> Self::Output 
    {
        match (self, rhs)
        {
            ( PortValue::Integer(a), PortValue::Integer(b) ) => PortValue::Integer( a * b),
            ( PortValue::Float(a), PortValue::Float(b) ) => PortValue::Float( a * b),
            ( PortValue::Float(a), PortValue::Integer(b) ) => PortValue::Float( a * b as f32),
            ( PortValue::Integer(a), PortValue::Float(b) ) => PortValue::Float( a as f32 * b),
            _ => panic!("Tried to do multiplication on two incompatible types"),
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
