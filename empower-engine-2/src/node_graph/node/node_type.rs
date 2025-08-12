// use std::fmt;

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
pub enum NodeType
{
    #[default] Start,
    Number,
    Addition,
    Multiply,
    Text,
    Bool,
    Print,
}

// impl fmt::Display for NodeType
// {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result 
//     {
//         write!(f, "{:?}", self)
//     }
// }
