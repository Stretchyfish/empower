#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum NodeType
{
    Start,
    #[default] IntegerVariable,
}
