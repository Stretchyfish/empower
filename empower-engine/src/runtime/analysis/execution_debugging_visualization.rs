use super::super::EmpowerExecutor;
use super::detect_execution_order;

use crate::analyser::node_graph_quick_overview;

pub fn start_debugging(executor: &mut EmpowerExecutor)
{   
    println!("------------------------ Empower Runtime Debugging -----------------------");
    println!("Node Graph View: ");
    node_graph_quick_overview(&executor.node_graph);
    println!("Execution order: {:?}", detect_execution_order(&mut executor.node_graph));
    println!("--------------------------------------------------------------------------");
}

pub fn runtime_debugging(executor: &mut EmpowerExecutor)
{
    if !executor.is_running()
    {
        return;
    }

    let current_execution_node_key= executor.execution_queue[0];
    let current_execute_node = executor.node_graph.nodes.get(&current_execution_node_key).unwrap();

    println!("Execution queue: {:?}", executor.execution_queue);

    println!("------------ {} ------------", current_execute_node.kind.name());
    println!("|                                |");

    let mut index = 0;

    while index < current_execute_node.input_port_keys.len() || index < current_execute_node.output_port_keys.len()
    {
        let mut input_text = String::from("|");

        if index < current_execute_node.input_port_keys.len() 
        {
            let input_port = executor.node_graph.input_ports.get(&current_execute_node.input_port_keys[index]).unwrap();
            input_text = format!("{}", input_port.value);
        }

        let mut output_text = String::from("|");
        if index < current_execute_node.output_port_keys.len() 
        {
            let output_port = executor.node_graph.output_ports.get(&current_execute_node.output_port_keys[index]).unwrap();
            output_text = format!("{}", output_port.value);
        }

        println!("{}                                {}", input_text, output_text);

        index += 1;
    }

    println!("|                                |");
    println!("----------------------------------");
}
