use egui;

use better_empower_engine::PortValue;

mod display_trigger_value;
use display_trigger_value::DisplayTriggerValue;

#[derive(Clone, PartialEq)]
pub enum DisplayValue
{
    Nothing,
    Text(String),
    Checkbox(bool),
}

impl DisplayValue
{
    pub fn from_port_value(port_value: &PortValue) -> Self
    {
        match port_value
        {
            PortValue::Trigger => Self::Nothing,
            PortValue::Integer( int ) => Self::Text( int.to_string() ),
            PortValue::Float( float ) => Self::Text( float.to_string() ),
            PortValue::Text(_) => Self::Text( String::new() ),
            PortValue::Bool( boolean) => Self::Checkbox( *boolean ),
            PortValue::Vector(port_values) => Self::Nothing,
            PortValue::None => Self::Nothing,
        }
    }

    pub fn to_port_value(&self, port_value: &PortValue) -> Option<PortValue>
    {
        let parsed_value = match (self, port_value)
        {
            (DisplayValue::Text( text_to_parse ), PortValue::Integer(_)) => self.text_to_int(&text_to_parse),
            (DisplayValue::Text( text_to_parse ), PortValue::Float(_)) => self.text_to_float(&text_to_parse),
            (DisplayValue::Text( text_send ), PortValue::Text(_)) => Some( PortValue::Text( text_send.to_string() ) ),
            (DisplayValue::Checkbox( boolean_to_parse), PortValue::Bool(_)) => Some( PortValue::Bool( *boolean_to_parse )),
            _ => panic!("Tried to pass two incompatible values"),
        };
        return parsed_value;
    }

    pub fn text_to_int(&self, text_to_parse: &String) -> Option<PortValue>
    {
        let parse_result = text_to_parse.parse::<i32>();
        match parse_result
        {
            Ok( parsed_value) => return Some( PortValue::Integer( parsed_value ) ),
            Err(_) => None,
        }
    }

    pub fn text_to_float(&self, text_to_parse: &String) -> Option<PortValue>
    {
        let parse_result = text_to_parse.parse::<f32>();
        match parse_result
        {
            Ok( parsed_value ) => return Some( PortValue::Float( parsed_value ) ),
            Err(_) => None,
        }
    }
}

// pub fn from_port_value()
// {

// }

// pub fn convert_display_value_to_port_value(display_value: DisplayValue, port_value: &PortValue) -> Result<PortValue, String>
// {
//     let updated_display_value = match (display_value, port_value)
//     {
//         (DisplayValue::Nothing, _) => todo!(), // This case should never happen! 
//         (DisplayValue::Text( text ), PortValue::Integer(_)) => {},
//         (DisplayValue::Text(_), PortValue::Trigger) => todo!(),
//         (DisplayValue::Text(_), PortValue::Integer(_)) => todo!(),
//         (DisplayValue::Text(_), PortValue::Float(_)) => todo!(),
//         (DisplayValue::Text(_), PortValue::Text(_)) => todo!(),
//         (DisplayValue::Text(_), PortValue::Bool(_)) => todo!(),
//         (DisplayValue::Text(_), PortValue::Vector(port_values)) => todo!(),
//         (DisplayValue::Text(_), PortValue::None) => todo!(),
//         (DisplayValue::Checkbox(_), PortValue::Trigger) => todo!(),
//         (DisplayValue::Checkbox(_), PortValue::Integer(_)) => todo!(),
//         (DisplayValue::Checkbox(_), PortValue::Float(_)) => todo!(),
//         (DisplayValue::Checkbox(_), PortValue::Text(_)) => todo!(),
//         (DisplayValue::Checkbox(_), PortValue::Bool(_)) => todo!(),
//         (DisplayValue::Checkbox(_), PortValue::Vector(port_values)) => todo!(),
//         (DisplayValue::Checkbox(_), PortValue::None) => todo!(),
//     }
// }

// impl DisplayValue
// {
//     pub fn can_convert_to_port_value(&self, port_value: &PortValue) -> bool
//     {

//         true
//     }
// }

pub trait DisplayValueKind
{
    fn text(&self) -> String;
    fn get_color(&self) -> egui::Color32;
    fn show(&mut self, ui: &mut egui::Ui, input_port_position: &egui::Pos2);
}

pub fn create_display_value_from_port_value(port_value: &PortValue)
{
    match port_value
    {
        PortValue::Trigger => todo!(),
        PortValue::Integer(_) => todo!(),
        PortValue::Float(_) => todo!(),
        PortValue::Text(_) => todo!(),
        PortValue::Bool(_) => todo!(),
        PortValue::Vector(port_values) => todo!(),
        PortValue::None => todo!(),
    }
}