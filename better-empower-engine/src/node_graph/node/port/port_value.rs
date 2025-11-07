use std::mem::discriminant;

#[derive(Default, Clone, Debug, PartialEq)]
pub enum PortValue
{
    Trigger,
    Integer(i32),
    Float(f32),
    Text(String),
    Bool(bool),
    Vector(Vec<PortValue>),
    #[default] None,
}

impl PortValue
{
    pub fn is_same_type_as(&self, value: &PortValue) -> bool
    {
        discriminant(self) == discriminant(value)
    }
}
