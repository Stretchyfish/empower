use crate::EmpowerData;

#[derive(Default, Clone)]
pub enum PortType
{
    Exatch(EmpowerData),
    OneOf(Vec<EmpowerData>),
    #[default] None, // @TODO, consider if there should be no none later
}

impl PortType
{
    pub fn get_compatability_list(&self) -> Vec<EmpowerData>
    {
        match self
        {
            PortType::Exatch( value ) =>
            {
                vec![ value.clone() ] 
            }

            PortType::OneOf( value_list ) =>
            {
                value_list.clone()
            }

            PortType::None =>
            {
                Vec::new()
            }
        }
    }

    pub fn is_compatible_with(&self, value: &EmpowerData) -> bool
    {
        match self 
        {
            PortType::Exatch( self_exact_type) =>
            {
                self_exact_type == value
            },

            PortType::OneOf( self_compatible_list_types ) =>
            {
                self_compatible_list_types.contains(value)
            },

            PortType::None =>
            {
                false
            },
        }
    }
}
