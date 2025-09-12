use egui;
use empower_engine::node_graph::port::{port_value::{self, PortValue}, PortCompatability};

#[derive(Default, Clone, PartialEq)]
pub struct DisplayPortValue
{
    pub text: String, // @TODO, make not string?
    pub value_type: DisplayPortValueType,
    pub display_value_valid: bool,
    pub color: egui::Color32, // @TODO, consider if this should be calculated
}

impl DisplayPortValue
{
    pub fn from(text: String, port_value: &PortValue) -> Self
    {
        let value_type = DisplayPortValueType::new(port_value);

        DisplayPortValue { text, value_type, display_value_valid: true, color: DisplayPortValue::get_port_color(port_value) }
    }

    fn get_port_color(port_value: &PortValue) -> egui::Color32
    {
        match port_value
        {
            PortValue::Trigger => egui::Color32::WHITE,
            PortValue::Integer(_) => egui::Color32::YELLOW,
            PortValue::Float(_) => todo!(),
            PortValue::Text(_) => egui::Color32::YELLOW,
            PortValue::Bool(_) => egui::Color32::YELLOW,
            PortValue::Undefined(_) => todo!(),
            PortValue::None => todo!(),
        }
    }

    pub fn nothing(port_value: &PortValue) -> Self
    {
        DisplayPortValue { text: "".to_string(), value_type: DisplayPortValueType::None, display_value_valid: true, color: DisplayPortValue::get_port_color(port_value) }
    }

    pub fn nothing_with_text(text: String, port_value: &PortValue) -> Self
    {
        DisplayPortValue { text, value_type: DisplayPortValueType::None, display_value_valid: true, color: DisplayPortValue::get_port_color(port_value) }
    }
    
    pub fn to(&self, compatabilities: &PortCompatability) -> Option<PortValue>
    {
        let compatability_list = compatabilities.get_compatability_list();

        for compatible_value in compatability_list
        {
            let parsed_value= match (self.value_type.clone(), compatible_value)
            {
                (DisplayPortValueType::Text( text_to_parse ), PortValue::Integer(_)) => self.text_to_int(&text_to_parse),
                (DisplayPortValueType::Text( text_to_parse ), PortValue::Float(_)) => self.text_to_float(&text_to_parse),
                (DisplayPortValueType::Text( text_send ), PortValue::Text(_)) => Some( PortValue::Text( text_send ) ),
                (DisplayPortValueType::Checkbox( boolean_to_parse), PortValue::Bool(_)) => Some( PortValue::Bool( boolean_to_parse )),
                _ => panic!("Tried to pass two incompatible values"),
            };

            if parsed_value.is_none()
            {
                continue;
            }

            return parsed_value
        }

        None
    }

    // maybe make the nodes go to the None DisplayPortType when connected to this statement?
    // Atleast think about it 
    pub fn update(&mut self, port_value: &PortValue)
    {
        // @TODO, consider if this is the best approach
        let updated_display_port_value_type = match (&self.value_type, port_value)
        {
            (DisplayPortValueType::Text(_), PortValue::Trigger) => todo!(),
            (DisplayPortValueType::Text(_), PortValue::Integer( int )) => DisplayPortValueType::Text( int.to_string() ),
            (DisplayPortValueType::Text(_), PortValue::Float( float )) => DisplayPortValueType::Text( float.to_string() ),
            (DisplayPortValueType::Text(_), PortValue::Text( text )) => DisplayPortValueType::Text( text.clone() ),
            (DisplayPortValueType::Text(_), PortValue::Bool( bool )) => DisplayPortValueType::Text( bool.to_string() ),
            (DisplayPortValueType::Text(_), PortValue::Undefined(_)) => todo!(),
            (DisplayPortValueType::Text(_), PortValue::None) => todo!(),
            (DisplayPortValueType::Checkbox(_), PortValue::Trigger) => todo!(),
            (DisplayPortValueType::Checkbox(_), PortValue::Integer(_)) => todo!(),
            (DisplayPortValueType::Checkbox(_), PortValue::Float(_)) => todo!(),
            (DisplayPortValueType::Checkbox(_), PortValue::Text(_)) => todo!(),
            (DisplayPortValueType::Checkbox(_), PortValue::Bool( bool )) => DisplayPortValueType::Checkbox( *bool ),
            (DisplayPortValueType::Checkbox(_), PortValue::Undefined(_)) => todo!(),
            (DisplayPortValueType::Checkbox(_), PortValue::None) => todo!(),
            (DisplayPortValueType::None, PortValue::Trigger) => DisplayPortValueType::None,
            (DisplayPortValueType::None, PortValue::Integer(_)) => DisplayPortValueType::None,
            (DisplayPortValueType::None, PortValue::Float(_)) => DisplayPortValueType::None,
            (DisplayPortValueType::None, PortValue::Text(_)) => DisplayPortValueType::None,
            (DisplayPortValueType::None, PortValue::Bool(_)) => DisplayPortValueType::None,
            (DisplayPortValueType::None, PortValue::Undefined(_)) => todo!(),
            (DisplayPortValueType::None, PortValue::None) => todo!(),
        };

        self.value_type = updated_display_port_value_type;
    }

    // @TODO, look at this again
    fn text_to_int(&self, text_to_parse: &String) -> Option<PortValue>
    {
        let parse_result = text_to_parse.parse::<i32>();
        match parse_result
        {
            Ok( parsed_value) => return Some( PortValue::Integer( parsed_value ) ),
            Err(_) => None,
        }
    }

    fn text_to_float(&self, text_to_parse: &String) -> Option<PortValue>
    {
        let parse_result = text_to_parse.parse::<f32>();
        match parse_result
        {
            Ok( parsed_value ) => return Some( PortValue::Float( parsed_value ) ),
            Err(_) => None,
        }
    }
}

#[derive(Default, Clone, PartialEq)]
pub enum DisplayPortValueType
{
    Text(String),
    Checkbox(bool),
    #[default] None, //@TODO, consider if this should be named something else?
}

impl DisplayPortValueType
{
    pub fn new(port_value: &PortValue) -> Self
    {
        match port_value
        {
            PortValue::Trigger => DisplayPortValueType::None,
            PortValue::Integer( int_value ) => DisplayPortValueType::Text( int_value.to_string() ),
            PortValue::Float(_) => todo!(),
            PortValue::Text( text_value ) => DisplayPortValueType::Text( text_value.clone() ),
            PortValue::Bool( bool_value ) => DisplayPortValueType::Checkbox( *bool_value ), 
            PortValue::Undefined(_) => todo!(),
            PortValue::None => todo!(),
        }
    }
}

// @TODO, decide if this should be used
pub fn get_display_port_value_color(display_value: &DisplayPortValue) -> egui::Color32
{
    match display_value.value_type
    {
        DisplayPortValueType::None => egui::Color32::GRAY,
        DisplayPortValueType::Checkbox(_) => egui::Color32::BLUE,
        DisplayPortValueType::Text(_) => egui::Color32::YELLOW, 
    }
}

// pub fn show_display_port_value(display_value_type: &DisplayPortValueType, input_port_text_position: &egui::Pos2, port_has_connection: bool)
pub fn show_display_port_value(display_value: &DisplayPortValue, input_port_position: &egui::Pos2, port_has_connection: bool, ui: &mut egui::Ui) -> DisplayPortValue
{
    let mut modified_display_value = display_value.clone(); // @TODO, find a better name

    let input_port_text_offset = egui::Vec2 { x: 40.0, y: 0.0};
    let input_port_text_position = *input_port_position + input_port_text_offset;

    let input_port_text_font_size = 35.0;

    let painted_text = ui.painter().text(
        input_port_text_position,
        // egui::Align2::LEFT_TOP,
        egui::Align2::LEFT_CENTER,
        &display_value.text,
        egui::FontId::proportional(input_port_text_font_size),
        egui::Color32::WHITE,
    );

    let painted_text_size = painted_text.size();
    let text_and_display_value_buffer = 20.0;

    let input_port_value_position = input_port_text_position + egui::Vec2 { x: painted_text_size.x + text_and_display_value_buffer, y: -painted_text_size.y / 2.0 };

    match &mut modified_display_value.value_type
    {
        DisplayPortValueType::None => (),
        DisplayPortValueType::Checkbox( toggle ) => 
        {
            let input_port_checkbox_size = egui::Vec2{ x: 120.0, y: 0.0 };
            let input_port_checkbox_rect = egui::Rect::from_min_size(input_port_value_position, input_port_checkbox_size);

            // @TODO, improve this, and fix box size
            let checkbox = egui::Checkbox::new(
                                                    toggle, 
                                                        egui::RichText::new("").font(egui::FontId::proportional(35.0)));
            ui.put(input_port_checkbox_rect, checkbox);
        },
        DisplayPortValueType::Text( text ) => 
        {
            let input_port_value_box_size = egui::Vec2{ x: 120.0, y: painted_text_size.y };
            let input_port_value_box_rect = egui::Rect::from_min_size(input_port_value_position, input_port_value_box_size);

            let mut text_edit_color = egui::Color32::WHITE;
            let mut text_background_color = egui::Color32::BLACK;

            if port_has_connection
            {
                text_edit_color = egui::Color32::GRAY;
                text_background_color = egui::Color32::TRANSPARENT;
            }

            if !display_value.display_value_valid
            {
                text_edit_color = egui::Color32::RED;
            }

            let text_edit = egui::TextEdit::singleline(text)
            .char_limit(5)
            .font(egui::FontId::proportional(35.0))
            .interactive(!port_has_connection)
            .text_color(text_edit_color)
            .background_color(text_background_color);

            ui.put(input_port_value_box_rect, text_edit);
        },
    }

    modified_display_value
}