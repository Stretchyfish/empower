use crate::EmpowerKey;

pub mod node_type;
pub use node_type::NodeType;

#[derive(Default, Clone)]
pub struct Node
{
    pub key: EmpowerKey,
    pub node_type: NodeType,
    pub input_port_keys: Vec<EmpowerKey>,
    pub output_port_keys: Vec<EmpowerKey>,
}

impl Node
{
    pub fn new(key: EmpowerKey, node_type: NodeType, input_port_keys: Vec<EmpowerKey>, output_port_keys: Vec<EmpowerKey>) -> Self
    {
        Self
        {
            key,
            node_type,
            input_port_keys,
            output_port_keys,
        }
    }
}
