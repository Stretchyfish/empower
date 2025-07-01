pub type EmpowerKey = i32;

pub mod empower_node_graph;
pub use empower_node_graph::EmpowerNodeGraph;

pub mod empower_data;
pub use empower_data::EmpowerData;

pub mod node;
pub use node::Node;
pub use node::NodeType;

pub mod port;
pub use port::input_port::InputPort;
pub use port::output_port::OutputPort;
