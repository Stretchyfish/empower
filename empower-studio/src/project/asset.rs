use std::path::PathBuf;
use std::fmt;

pub type AssetId = i32;

#[derive(Clone)]
pub struct Asset // @TODO, maybe to follow convention name it asset meta?
{
    pub id: AssetId,
    pub path: PathBuf,
    pub kind: AssetKind,
}

#[derive(Clone, Copy)]
pub enum AssetKind
{
    None,
    Folder,
    Text,
    Image,
}

impl fmt::Display for AssetKind
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result 
    {
        write!(f, "{}", self)
    }
}
