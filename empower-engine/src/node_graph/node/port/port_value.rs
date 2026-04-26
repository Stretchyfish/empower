use std::mem::discriminant;
use std::fmt;
use std::ops;

#[derive(Default, Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum PortValue
{
    Trigger(bool),
    Integer(i32),
    Float(f32),
    Text(String),
    Bool(bool),
    Vector(Vec<PortValue>),
    Range(i32, i32, i32),
    #[default] None,
}

impl PortValue
{
    pub fn is_same_type_as(&self, value: &PortValue) -> bool
    {
        discriminant(self) == discriminant(value)
    }

    pub fn type_name(&self) -> &'static str
    {
        match self
        {
            PortValue::Trigger(_) => "trigger",
            PortValue::Integer(_) => "integer",
            PortValue::Float(_) => "float",
            PortValue::Text(_) => "text",
            PortValue::Bool(_) => "bool",
            PortValue::Vector(_) => "vector",
            PortValue::Range(_, _, _) => "range",
            PortValue::None => "none",
        }
    }
}

impl fmt::Display for PortValue
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        match self
        {
            PortValue::Trigger(value) => write!(f, "trigger {}", value),
            PortValue::Integer(value) => write!(f, "{}", value),
            PortValue::Float(value) => write!(f, "{}", value),
            PortValue::Text(value) => write!(f, "{}", value),
            PortValue::Bool(value) => write!(f, "{}", value),
            PortValue::None => write!(f, "none"),
            PortValue::Vector(port_values) => write!(f, "{:?}", port_values),
            PortValue::Range(from, interval, to) => write!(f, "{:?},{:?},{:?}", from, interval, to),
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

impl PartialOrd for PortValue
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {

        match (self, other)
        {
            ( PortValue::Integer(a), PortValue::Integer(b)) => a.partial_cmp(b),
            ( PortValue::Integer(a), PortValue::Float(b)) => (*a as f32).partial_cmp(b),
            ( PortValue::Float(a), PortValue::Integer(b)) => a.partial_cmp(&(*b as f32)),
            ( PortValue::Float(a), PortValue::Float(b)) => a.partial_cmp(b),
            _ => panic!("Tried to compare on two incompatible types"),
        }
    }
}
