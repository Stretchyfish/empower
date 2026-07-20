use crate::value::Value;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum PortEdit
{
    None,
    Text(String),
    CheckBox(bool),
    TwoBox(String, String),
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
            PortEdit::TwoBox( text1, text2 ) => attempt_to_parse_two_box(text1, text2),
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

fn attempt_to_parse_two_box(text1: &String, text2: &String) -> Option<Value>
{
    let parsed1 = text1.parse::<f32>();

    if parsed1.is_err()
    {
        return None;
    }

    let parsed2 = text2.parse::<f32>();

    if parsed2.is_err()
    {
        return None;
    }

    return Some( Value::Point2d( parsed1.unwrap(), parsed2.unwrap() ) );
}
