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
}
