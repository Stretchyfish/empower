use better_empower_engine::{NodeGraph, PortValue, EmpowerRuntime};

fn main()
{
    let mut node_graph = NodeGraph::new();

    let _ = node_graph.add_node("start");
    let number_node_1 = node_graph.add_node("number");

    node_graph.set_input_port_value(&number_node_1.input_port_keys[0], PortValue::Integer(5));

    let number_node_2 = node_graph.add_node("number");
    let addition_node = node_graph.add_node("addition");
    let multiply_node = node_graph.add_node("multiply");

    let connection_response = node_graph.add_connection(number_node_1.output_port_keys[0], number_node_2.input_port_keys[0]);
    let _ = node_graph.add_connection(number_node_1.output_port_keys[0], addition_node.input_port_keys[0]);
    let _ = node_graph.add_connection(number_node_2.output_port_keys[0], addition_node.input_port_keys[1]);
    let _ = node_graph.add_connection(number_node_1.output_port_keys[0], multiply_node.input_port_keys[0]);
    let _ = node_graph.add_connection(number_node_2.output_port_keys[0], multiply_node.input_port_keys[1]);

    if connection_response.is_err()
    {
        println!("{}", connection_response.err().unwrap());
    }

    let mut runtime = EmpowerRuntime::new(node_graph, true);
    runtime.execute();
}