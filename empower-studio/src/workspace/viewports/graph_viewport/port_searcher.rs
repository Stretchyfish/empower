use empower_node_graph::{EmpowerData, EmpowerKey};

#[derive(Clone, Copy)]
pub struct PortSearcher
{
    pub port_key: EmpowerKey,
    pub port_kind: PortKind,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PortKind
{
    InputPort,
    OutputPort,
}
