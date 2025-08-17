use std::fmt;

use super::PortValue;

#[derive(Default, Clone)]
pub enum PortCompatability
{
    Exatch(PortValue),
    OneOf(Vec<PortValue>),
    Any,
    #[default] None,
}

impl PortCompatability
{
    pub fn get_initial_port_value(&self) -> PortValue
    {
        match self 
        {
            PortCompatability::Exatch( extact_port_value ) => extact_port_value.clone(),
            PortCompatability::OneOf( one_of_port_values ) =>
            {
                let mut port_value = PortValue::None;
                if !one_of_port_values.is_empty()
                {
                    port_value = one_of_port_values[0].clone();
                }
                port_value
            }
            PortCompatability::Any => PortValue::Undefined( String::new() ), 
            PortCompatability::None => PortValue::None,
        }
    }

    pub fn get_compatability_list(&self) -> Vec<PortValue>
    {
        match self
        {
            PortCompatability::Exatch( value ) => vec![ value.clone() ], 
            PortCompatability::OneOf( value_list ) => value_list.clone(),
            PortCompatability::Any => Vec::new(),
            PortCompatability::None => Vec::new()
        }
    }


}

impl fmt::Display for PortCompatability
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result 
    {
        let compatability_list = self.get_compatability_list();
        write!(f, "{:?}", compatability_list)
    }
}
