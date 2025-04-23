use crate::EmpowerKey;

#[derive(Default, Clone, Copy)]
pub struct InputPort
{
    pub key: EmpowerKey,
    pub value: i32,
}

impl InputPort
{
    pub fn new(key: EmpowerKey) -> Self
    {
        Self
        {
            key,
            value: 0,
        }
    }
}
