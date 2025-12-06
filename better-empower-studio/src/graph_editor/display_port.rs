use egui;

pub struct DisplayPort
{
    pub text: &'static str,
    pub position: egui::Pos2,
}

impl DisplayPort
{
    pub fn new(text: &'static str, position: egui::Pos2) -> Self
    {
        Self { text, position }
    }
}