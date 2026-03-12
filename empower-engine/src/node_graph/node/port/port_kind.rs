#[derive(Default, Clone, serde::Serialize, serde::Deserialize)]
pub enum PortKind
{
    #[default] Input,
    Output,
}
