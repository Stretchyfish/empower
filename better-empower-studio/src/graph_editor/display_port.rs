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
    pub fn new(text: &'static str, position: egui::Pos2) -> Self
    {
        // Self { text, position, value: DisplayValue::Nothing }
        // Self { text, position, value: DisplayValue::checkbox(true) }
        Self { text, position, value: DisplayValue::Text(String::new()), convertable: true }
    }

    pub fn try_to_convert_to_port_value(&mut self, port_value: &PortValue)
    {

    }

}