use std::fmt;

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
pub enum NodeKind
{
    #[default] Start,
    Number,
    Addition,
    Multiply,
    Text,
    Bool,
    Print,
}

impl fmt::Display for NodeKind
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result 
    {
        write!(f, "{:?}", self)
    }
}
