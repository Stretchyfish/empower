use better_empower_engine::{NodeGraph, PortValue, EmpowerRuntime};

fn main()
{
    let mut node_graph = NodeGraph::new();

    let start_node = node_graph.add_node("start");
    let image_node = node_graph.add_node("show image");
    node_graph.set_input_port_value(&image_node.input_port_keys[1], PortValue::Text( String::from("/home/mikkel/Downloads/cat.webp")));

    let _ = node_graph.add_connection(start_node.output_port_keys[0], image_node.input_port_keys[0]);

    let mut runtime = EmpowerRuntime::new(node_graph, true);
    runtime.execute();
}
