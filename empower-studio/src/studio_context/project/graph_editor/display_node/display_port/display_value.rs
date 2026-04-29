use empower_engine::{PortValue, node_graph::node::port::PortCompatability};

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum DisplayValue
{
    Nothing,
    Text(String),
    Checkbox(bool),
    Interval(String, String, String),
}

impl DisplayValue
{
    pub fn from_port_value(port_value: &PortValue) -> Self
    {
        match port_value
        {
            PortValue::Trigger(_) => Self::Nothing,
            PortValue::Integer( int ) => Self::Text( int.to_string() ),
            PortValue::Float( float ) => Self::Text( float.to_string() ),
            PortValue::Text(_) => Self::Text( String::new() ),
            PortValue::Bool( boolean) => Self::Checkbox( *boolean ),
            PortValue::Vector(_) => Self::Nothing,
            PortValue::Range( from, interval, to ) => Self::Interval( from.to_string(), interval.to_string(), to.to_string() ), 
            PortValue::None => Self::Nothing,
        }
    }

    // Converting to a port value should be done using port compatability, because even if the current value fails,
    // another one might be compatible
    pub fn to_port_value(&self, port_compatabilities: &PortCompatability) -> Option<PortValue>
    {
        for compatible_value in port_compatabilities.get_compatability_list()
        {
            let parsed_value = match (self, compatible_value)
            {
                (DisplayValue::Text( text_to_parse ), PortValue::Integer(_)) => self.text_to_int(&text_to_parse),
                (DisplayValue::Text( text_to_parse ), PortValue::Float(_)) => self.text_to_float(&text_to_parse),
                (DisplayValue::Text( text_send ), PortValue::Text(_)) => Some( PortValue::Text( text_send.to_string() ) ),
                (DisplayValue::Checkbox( boolean_to_parse), PortValue::Bool(_)) => Some( PortValue::Bool( *boolean_to_parse )),
                (DisplayValue::Interval( from_str, interval_str, to_str), PortValue::Range(_, _, _)) =>
                {
                    let from = match from_str.parse::<i32>()
                    {
                        Ok( parsed_value) => Some( parsed_value ),
                        Err(_) => None,
                    };

                    let interval = match interval_str.parse::<i32>()
                    {
                        Ok( parsed_value) => Some( parsed_value ),
                        Err(_) => None,
                    };

                    let to = match to_str.parse::<i32>()
                    {
                        Ok( parsed_value) => Some( parsed_value ),
                        Err(_) => None,
                    };

                    if from.is_none() || interval.is_none() || to.is_none()
                    {
                        return None;
                    }

                    Some( PortValue::Range( from.unwrap(), interval.unwrap(), to.unwrap() ) )
                },
                _ => panic!("Tried to pass two incompatible values"),
            };

            if parsed_value.is_none()
            {
                continue;
            }

            return parsed_value;
        }
 
        None
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
