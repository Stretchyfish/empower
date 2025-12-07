use better_empower_engine::NodeGraphKey;
pub use better_empower_engine::node_graph::node::port::PortKind;

#[derive(Clone)]
pub struct PortSearcher
{
    pub port_key: NodeGraphKey,
    pub port_kind: PortKind, 
}

impl PortSearcher
{
    pub fn input_port_searching(port_key: NodeGraphKey) -> Self
    {
        PortSearcher { port_key, port_kind: PortKind::Input }
    }

    pub fn output_port_searching(port_key: NodeGraphKey) -> Self
    {
        PortSearcher { port_key, port_kind: PortKind::Output }
    }
}
