use better_empower_engine::PortValue;
use egui;

use super::display_value::DisplayValue;

#[derive(Clone, PartialEq)]
pub struct DisplayPort
{
    pub text: &'static str,
    pub position: egui::Pos2,
    pub value: DisplayValue,
    pub convertable: bool,
}

impl DisplayPort
{
    pub fn new(text: &'static str, position: egui::Pos2, port_value: &PortValue) -> Self
    {
        Self { text, position, value: DisplayValue::from_port_value(port_value), convertable: true }
    }

    pub fn nothing(position: egui::Pos2) -> Self
    {
        Self { text: "", position, value: DisplayValue::Nothing, convertable: true }
    }
}