use crate::EmpowerKey;

#[derive(Default, Clone)]
pub struct OutputPort
{
    pub key: EmpowerKey,
    pub value: i32,
}

impl OutputPort
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
