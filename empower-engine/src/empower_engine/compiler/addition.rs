use empower_node_graph::port::input_port;
use empower_node_graph::EmpowerKey;
use empower_node_graph::EmpowerData;
use empower_node_graph::Node;
use empower_node_graph::NodeType;
use empower_node_graph::InputPort;
use empower_node_graph::OutputPort;
use std::collections::HashMap;

pub fn execute_debug_addition_node(node_to_compile_key: EmpowerKey, nodes: &mut HashMap<EmpowerKey, Node>, input_ports: &mut HashMap<EmpowerKey, InputPort>, output_ports: &mut HashMap<EmpowerKey, OutputPort>)
{
    let node_to_compile = nodes.get_mut(&node_to_compile_key).unwrap(); // @TODO, handle exception

    if node_to_compile.node_type != NodeType::Addition
    {
        println!("Asked to execute addition node, but its type does't match, request will be ignored");
        return;
    }

    if node_to_compile.input_port_keys.len() != 2
    {
        println!("Asked to execute addition node, but its input ports size is invalid, request will be ignored");
        return;
    }

    if node_to_compile.output_port_keys.len() != 1
    {
        println!("Asked to execute addition node, but its output ports size is invalid, request will be ignored");
        return;
    }

    let input_port_1 = input_ports.get(&node_to_compile.input_port_keys[0]).unwrap(); // @TODO, handle exception
    let input_port_2 = input_ports.get(&node_to_compile.input_port_keys[1]).unwrap(); // @TODO, handle exception

    // if input_port_1.value != input_port_2.value
    // {
    //     println!("Addition node has two different incompatable numbers");
    //     return;
    // }

    let mut added_value = EmpowerData::Unknown; // @TODO, consider making this not mut?
    match input_port_1.value // @TODO, replace this with operator overloading?
    {
        EmpowerData::Integer( integer ) => // @TODO, simplify this whole chain
        {
            match input_port_2.value
            {
                EmpowerData::Integer( integer2 ) =>
                {
                    added_value = EmpowerData::Integer( integer + integer2);
                },
                EmpowerData::Float( float2 ) =>
                {
                    added_value = EmpowerData::Float( integer as f32 + float2 ); // @TODO, consider if this is a bad appraoch?
                },
                _ =>
                {

                }
            }
        },

        EmpowerData::Float( float ) =>
        {
            match input_port_2.value
            {
                EmpowerData::Integer( integer2 ) =>
                {
                    added_value = EmpowerData::Float( float + integer2 as f32 );
                },
                EmpowerData::Float( float2 ) =>
                {
                    added_value = EmpowerData::Float( float + float2 );
                },
                _ =>
                {

                },
            }
        }
        
        _ =>
        {
            
        }
       
    }

    let output_port = output_ports.get_mut(&node_to_compile.output_port_keys[0]).unwrap();
    output_port.value = added_value;

    print!("addition node ({}) : [{}] [{}], [{}]", node_to_compile_key, input_port_1.value, input_port_2.value, output_port.value);
}
