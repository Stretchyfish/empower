use crate::EmpowerKey;
use crate::EmpowerData;

#[derive(Default, Clone, Copy)]
pub struct InputPort
{
    pub key: EmpowerKey,
    pub value: EmpowerData,
}

impl InputPort
{
    pub fn new(key: EmpowerKey) -> Self
    {
        Self
        {
            key,
            value: EmpowerData::Integer(0),
        }
    }

    // @TODO, determine if these should be somewhere different
    pub fn get_value_as_string(&self) -> String
    {
        self.value.to_string()
    }

    pub fn set_value_with_text(&mut self, value_text: &String) -> bool
    {
        match self.value 
        {
            EmpowerData::Integer(_) => 
            {
                self.value = match value_text.parse::<i32>()
                {
                   Ok(integer) => EmpowerData::Integer(integer),
                   Err(e) => return false,
                };
            },
            EmpowerData::Unknown =>
            {
                println!("ERROR, tried to set value with text of an unknown type");
            },
        }

        true
    }
}
