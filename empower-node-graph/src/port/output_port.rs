use crate::EmpowerKey;
use crate::EmpowerData;

#[derive(Default, Clone)]
pub struct OutputPort
{
    pub key: EmpowerKey,
    pub value: EmpowerData,
}

impl OutputPort
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
