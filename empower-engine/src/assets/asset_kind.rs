
#[derive(Clone, PartialEq, Eq)]
pub enum AssetKind
{
    Graph,
}

impl AssetKind
{
    pub fn to_string(&self) -> String
    {
        match self
        {
            AssetKind::Graph => "graph".to_string(),
        }
    }
}
