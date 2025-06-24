use empower_node_graph::EmpowerKey;
use super::PortKind;

#[derive(Clone, Copy)]
pub struct PortSearcher
{
    pub port_key: EmpowerKey,
    pub port_kind: PortKind,
}
