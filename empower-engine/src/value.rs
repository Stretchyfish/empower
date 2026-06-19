
#[derive(PartialEq, PartialOrd, Clone)]
pub enum Value
{
    Integer( i32 ),
    Float( f32 ),
}

impl Value
{
    pub fn to_string(&self) -> String
    {
        match self
        {
            Value::Integer( integer ) => integer.to_string(),
            Value::Float( float ) => float.to_string(),
        }
    }

    pub fn type_string(&self) -> String
    {
        match self
        {
            Value::Integer( _ ) => "integer".to_string(),
            Value::Float( _ ) => "float".to_string(),
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
}
