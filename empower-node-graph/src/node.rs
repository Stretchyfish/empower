use crate::EmpowerKey;

pub mod node_type;
pub use node_type::NodeType;

pub mod node_state;
pub use node_state::NodeState;

#[derive(Default, Clone)]
pub struct Node
{
    pub key: EmpowerKey,
    pub node_type: NodeType,
    pub state: NodeState,
    pub input_port_keys: Vec<EmpowerKey>,
    pub output_port_keys: Vec<EmpowerKey>,
}

impl Node
{
    pub fn new(key: EmpowerKey, node_type: NodeType, input_port_keys: Vec<EmpowerKey>, output_port_keys: Vec<EmpowerKey>) -> Self
    {

        let state = match node_type
        {
            NodeType::Start =>
            {
                NodeState::None
            }
            NodeType::Number =>
            {
                NodeState::NumberState
            }
            NodeType::Addition =>
            {
                NodeState::None
            }
            NodeType::IntegerVariable =>
            {
                NodeState::None
            }
            NodeType::Print =>
            {
                NodeState::None
            }
        };

        Self
        {
            key,
            node_type,
            state,
            input_port_keys,
            output_port_keys,
        }
    }
}
