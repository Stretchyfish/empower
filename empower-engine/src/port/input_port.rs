use crate::EmpowerKey;

#[derive(Default)]
pub struct InputPort
{
    key: EmpowerKey,
}

impl InputPort
{
    pub fn new(key: EmpowerKey) -> Self
    {
        Self
        {
            key,
        }
    }
}
