pub mod number_node_state;
pub use number_node_state::NumberNodeState;

#[derive(Default, Clone)]
pub enum NodeState
{
    NumberState(NumberNodeState),
    #[default] None,
}