use crate::value::Value;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum PortEdit
{
    None,
    Text(String),
    CheckBox(bool),
}

impl PortEdit
{
    pub fn convert_to_value(&self, compatabilities: &Vec<Value>) -> Option<Value>
    {
        match &self
        {
            PortEdit::None => None,
            PortEdit::Text( text ) => attempt_to_parse_text(text, compatabilities),
            PortEdit::CheckBox( toggle ) => Some( Value::Bool( *toggle ) ),
        }
    }
}

fn attempt_to_parse_text(text: &String, compatabilities: &Vec<Value>) -> Option<Value>
{
    for value in compatabilities
    {
        match value
        {
            Value::Integer(_) =>
            {
                let parsed = text.parse::<i32>();

                if parsed.is_err()
                {
                    continue;
                }

                return Some( Value::Integer( parsed.unwrap() ) );
            },
            Value::Float(_) =>
            {
                let parsed = text.parse::<f32>();

                if parsed.is_err()
                {
                    continue;
                }

                return Some( Value::Float( parsed.unwrap() ) );
            },
            _ => todo!(),
        }
    }

    None
}
