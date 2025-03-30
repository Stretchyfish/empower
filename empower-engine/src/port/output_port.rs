use crate::EmpowerKey;

#[derive(Default)]
pub struct OutputPort
{
    key: EmpowerKey
}

impl OutputPort
{
    pub fn new(key: EmpowerKey) -> Self
    {
        Self
        {
            key,
        }
    }
}
