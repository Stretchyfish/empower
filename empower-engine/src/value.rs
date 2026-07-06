use serde::{Deserialize, Serialize};

#[derive(PartialEq, PartialOrd, Clone, Serialize, Deserialize)]
pub enum Value
{
    Integer( i32 ),
    Float( f32 ),
    Bool( bool ),
    Image,
}

impl Value
{
    pub fn to_string(&self) -> String
    {
        match self
        {
            Value::Integer( integer ) => integer.to_string(),
            Value::Float( float ) => float.to_string(),
            Value::Bool( boolean ) => boolean.to_string(),
            Value::Image => "image".to_string(),
        }
    }

    pub fn type_string(&self) -> String
    {
        match self
        {
            Value::Integer( _ ) => "integer".to_string(),
            Value::Float( _ ) => "float".to_string(),
            Value::Bool( _ ) => "bool".to_string(),
            Value::Image => "image".to_string(),
        }
    }

    pub fn as_f32(&self) -> f32
    {
        match self
        {
            Value::Float( float ) => *float,
            _ => panic!("tried to convert impossible value to f32"),
        }
    }

    pub fn as_bool(&self) -> bool
    {
        match self
        {
            Value::Bool( boolean ) => *boolean,
            _ => panic!("tried to convert impossible value to bool"),
        }
    }
}
