use crate::port::port_type;
use crate::port::port_type::PortType;
use crate::EmpowerKey;
use crate::EmpowerData;

#[derive(Default, Clone)]
pub struct InputPort
{
    pub key: EmpowerKey,
    pub node_key: EmpowerKey,
    pub port_type: PortType,
    pub value: EmpowerData,
}

impl InputPort
{
    pub fn new(port_key: EmpowerKey, node_key: EmpowerKey, port_type: PortType, value: EmpowerData) -> Self
    {
        Self
        {
            key: port_key,
            node_key,
            port_type,
            value,
        }
    }

    pub fn reset(&mut self)
    {
        let compatability_list = self.port_type.get_compatability_list();

        if compatability_list.len() < 1
        {
            println!("tried to reset an input port without any compatability"); // This should never happen
            return;
        }

        self.value = compatability_list[0].clone();
    }
}
