use std::path::PathBuf;

pub type AssetId = i32;

pub struct Asset // @TODO, maybe to follow convention name it asset meta?
{
    id: AssetId,
    paht: PathBuf,
    kind: AssetKind,
}

pub enum AssetKind
{
    None,
    Text,
    Image,
}
