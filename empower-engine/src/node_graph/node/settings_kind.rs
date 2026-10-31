use crate::assets::{AssetId, AssetKind};

pub enum SettingsKind<'a>
{
    Text { label: &'a str, value: &'a mut String },
    FilePath { label: &'a str, value: &'a mut String },
    Integer { label: &'a str, value: &'a mut usize },
    Slider { label: &'a str, value: &'a mut f32, min: f32, max: f32 },
    Choice { label: &'a str, value: &'a mut String, options: Vec<&'a str> },
    Asset { label: &'a str, value: &'a mut Option<AssetId>, kind: AssetKind },
    OpenGraph { graph_id: AssetId },
}

impl SettingsKind<'_>
{
    pub fn height(&self) -> f32
    {
        match self
        {
            SettingsKind::Text { .. } => 50.0,
            SettingsKind::FilePath { .. } => 50.0,
            SettingsKind::Integer { .. } => 50.0,
            SettingsKind::Slider { .. } => 50.0,
            SettingsKind::Choice { .. } => 50.0,
            SettingsKind::Asset { .. } => 50.0,
            SettingsKind::OpenGraph { .. } => 50.0,
        }
    }
}
