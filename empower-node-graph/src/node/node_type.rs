use std::fmt;

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum NodeType
{
    Start,
    #[default] IntegerVariable,
    Addition
}

impl fmt::Display for NodeType
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result 
    {
        match *self
        {
            NodeType::IntegerVariable => write!(f, "Integer Variable"),
            NodeType::Addition => write!(f, "Addition"),
            NodeType::Start => write!(f, "Start"),
        }
    }
}
