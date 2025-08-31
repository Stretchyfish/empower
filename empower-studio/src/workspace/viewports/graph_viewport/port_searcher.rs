use empower_engine::NodeGraphKey;

#[derive(Clone, Copy)]
pub struct PortSearcher
{
    pub port_key: NodeGraphKey,
    pub port_kind: PortKind, 
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PortKind // @TODO, consider utilizing the already existing version of port kind in the engine
{
    InputPort,
    OutputPort,
}
