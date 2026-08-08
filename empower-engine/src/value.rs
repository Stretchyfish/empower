use serde::{Deserialize, Serialize};

use crate::assets::AssetId;

#[derive(PartialEq, PartialOrd, Debug, Clone, Serialize, Deserialize)]
pub enum Value
{
    Integer( i32 ),
    Float( f32 ),
    Bool( bool ),
    Image( Option<AssetId> ),
    Point2d ( f32, f32),
    List ( Vec<Value> ),
}

impl Value
{
    pub fn to_string(&self) -> String
    {
        match self
        {
            Value::Integer( integer ) => integer.to_string(),
            Value::Float( float ) => format!("{:?}", float), // This is to use the Debug methods instead, an maintain decimal points for whole numbers
            Value::Bool( boolean ) => boolean.to_string(),
            Value::Image( asset_id ) => if asset_id.is_none() { "none".to_string() } else { asset_id.unwrap().to_string() },
            Value::Point2d( x, y) => format!("[{},{}]", x, y),
            Value::List( _ ) => "list".to_string(),
        }
    }

    pub fn type_string(&self) -> String
    {
        match self
        {
            Value::Integer( _ ) => "integer".to_string(),
            Value::Float( _ ) => "float".to_string(),
            Value::Bool( _ ) => "bool".to_string(),
            Value::Image( _ ) => "image".to_string(),
            Value::Point2d( _, _) => "point2d".to_string(),
            Value::List( _ ) => "list".to_string(),
        }
    }

    pub fn from_type_string(type_string: &String) -> Option<Self>
    {
        match type_string.as_str()
        {
            "integer" => Some( Value::Integer(0) ),
            "float" => Some( Value::Float(0.0) ),
            "bool" => Some( Value::Bool(false) ),
            "image" => Some( Value::Image( None ) ),
            "point2d" => Some( Value::Point2d(0.0, 0.0) ),
            "list" => Some( Value::List( Vec::new() ) ),
            _ => None,
        }
    }

    pub fn as_i32(&self) -> i32
    {
        match self
        {
            Value::Integer( int ) => *int,
            _ => panic!("tried to convert impossible value to f32"),
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

    pub fn as_asset_id(&self) -> Option<AssetId>
    {
        match self
        {
            Value::Image( asset_id ) => *asset_id,
            _ => panic!("tried to convert impossible value to asset_id"),
        }
    }

    pub fn as_list(&self) -> Vec<Value>
    {
        match self
        {
            Value::List( list ) => list.clone(), // @TODO, this is potentially very expensive, consider a better way
            _ => panic!("tried to convert impossible value to list"),
        }
    }
}
