use crate::NodeGraph;

pub fn node_graph_quick_overview(node_graph: &NodeGraph)
{
    println!("----- Node Graph Overview -----");
    println!("nodes: {}", node_graph.node_count());

    for node in node_graph.get_all_nodes()
    {
        println!("--- key: {}, type: {}", node.key, node.node_type);
    }

    println!("input ports: {}", node_graph.input_port_count());

    for port in node_graph.get_all_input_ports()
    {
        println!("--- key: {}, node_key: {}, value: {:?}, compatability: {}", port.key, port.node_key, port.value, port.compatability);
    }

    println!("output ports: {}", node_graph.output_port_count());

    for port in node_graph.get_all_output_ports()
    {
        println!("--- key: {}, node_key: {}, value: {:?}, compatability: {}", port.key, port.node_key, port.value, port.compatability);
    }

    println!("--------------------------------");
}