use std::fmt;

use super::PortValue;

#[derive(Default, Clone, Debug)]
pub enum PortCompatability
{
    Exatch(PortValue),
    OneOf(Vec<PortValue>),
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
            PortCompatability::None => PortValue::None,
        }
    }

    pub fn get_compatability_list(&self) -> Vec<PortValue>
    {
        match self
        {
            PortCompatability::Exatch( value ) => vec![ value.clone() ], 
            PortCompatability::OneOf( value_list ) => value_list.clone(),
            PortCompatability::None => Vec::new()
        }
    }

    pub fn contains_port_value_type(&self, value_to_find: &PortValue) -> bool
    {
        match self
        {
            PortCompatability::Exatch( value ) => value.is_same_type_as(value_to_find),
            PortCompatability::OneOf( values ) =>
            {
                for value in values
                {
                    if value.is_same_type_as(value_to_find)
                    {
                        return true
                    }
                }

                false
            },
            PortCompatability::None => false, 
        }
    }

    pub fn is_compatible_with(&self, compatability_to_check: &PortCompatability) -> bool
    {
        let port_values_compatible = compatability_to_check.get_compatability_list();

        for port_value_to_check in port_values_compatible
        {
            if self.contains_port_value_type(&port_value_to_check)
            {
                return true;
            }
        }

        false
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
