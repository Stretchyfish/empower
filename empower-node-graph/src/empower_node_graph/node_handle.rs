use crate::EmpowerKey;

pub struct NodeHandle
{
    pub node_key: EmpowerKey,
    pub input_port_keys: Vec<EmpowerKey>,
    pub output_port_keys: Vec<EmpowerKey>,
}
