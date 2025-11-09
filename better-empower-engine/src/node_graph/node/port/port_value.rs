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

        match self
        {
            PortValue::Trigger => write!(f, "trigger"),
            PortValue::Integer(value) => write!(f, "{}", value),
            PortValue::Float(value) => write!(f, "{}", value),
            PortValue::Text(value) => write!(f, "{}", value),
            PortValue::Bool(value) => write!(f, "{}", value),
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
