use empower_engine::PortValue;
use egui;

pub mod display_value;
pub use display_value::DisplayValue;

#[derive(Clone, PartialEq)]
pub struct DisplayPort
{
    pub text: String,
    pub position: egui::Pos2,
    pub color: egui::Color32,
    pub value: DisplayValue,
    pub convertable: bool,
}

impl DisplayPort
{
    pub fn new(text: String, position: egui::Pos2, port_value: &PortValue) -> Self
    {
        Self { text, position, color: color_of_port_value(port_value), value: DisplayValue::from_port_value(port_value), convertable: true }
    }

    pub fn nothing(position: egui::Pos2, port_value: &PortValue) -> Self
    {
        Self { text: String::new(), position, color: color_of_port_value(port_value), value: DisplayValue::Nothing, convertable: true }
    }
}

fn color_of_port_value(port_value: &PortValue) -> egui::Color32
{
    match port_value 
    {
        PortValue::Trigger => egui::Color32::WHITE,
        PortValue::Integer(_) => egui::Color32::YELLOW,
        PortValue::Float(_) => egui::Color32::YELLOW,
        PortValue::Text(_) => egui::Color32::BLUE,
        PortValue::Bool(_) => egui::Color32::GREEN,
        PortValue::Vector(_) => egui::Color32::YELLOW,
        PortValue::None => egui::Color32::YELLOW,
    }
}
