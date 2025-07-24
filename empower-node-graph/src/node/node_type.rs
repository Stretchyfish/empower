use std::fmt::{self, write};

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum NodeType
{
    Start,
    #[default] IntegerVariable,
    Number,
    Addition,
    Multiply,
    Print,
}

impl fmt::Display for NodeType
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result 
    {
        match *self
        {
            NodeType::IntegerVariable => write!(f, "Integer Variable"),
            NodeType::Addition => write!(f, "Addition"),
            NodeType::Multiply => write!(f, "Multiply"),
            NodeType::Number => write!(f, "Number"),
            NodeType::Print => write!(f, "Print"),
            NodeType::Start => write!(f, "Start"),
        }
    }
}
