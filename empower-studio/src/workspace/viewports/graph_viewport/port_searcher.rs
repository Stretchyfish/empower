use empower_engine::NodeGraphKey;

#[derive(Clone, Copy)]
pub struct PortSearcher
{
    pub port_key: NodeGraphKey,
    pub port_kind: PortKind, 
}

impl PortSearcher
{
    pub fn input_port_searching(port_key: NodeGraphKey) -> Self
    {
        PortSearcher { port_key, port_kind: PortKind::InputPort }
    }

    pub fn output_port_searching(port_key: NodeGraphKey) -> Self
    {
        PortSearcher { port_key, port_kind: PortKind::OutputPort }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PortKind // @TODO, consider utilizing the already existing version of port kind in the engine
{
    InputPort,
    OutputPort,
}
