use empower_engine::{compiler::{CompileSettings, Instruction, compile}, node_graph::node::{NodeKind, node_kind::LoopMode}, project::Project, value::Value};

#[test]
fn test_loop_wait_print_compilation()
{
    let mut project = Project::new();

    {
        let entry_node_graph = project.assets.get_node_graph_mut(&project.entry_graph).unwrap();

        let start_node_handle = entry_node_graph.get_node_handle(entry_node_graph.start_node_key);
        let loop_node_handle = entry_node_graph.add_node(NodeKind::Loop( LoopMode::Forever ), None);
        let wait_node_handle = entry_node_graph.add_node(NodeKind::Wait, None);
        let print_node_handle = entry_node_graph.add_node(NodeKind::Print, None);

        let _ = entry_node_graph.add_connection(&start_node_handle.output_port_keys[0], &loop_node_handle.input_port_keys[0] );
        let _ = entry_node_graph.add_connection(&loop_node_handle.output_port_keys[0], &wait_node_handle.input_port_keys[0] );
        let _ = entry_node_graph.add_connection(&wait_node_handle.output_port_keys[0], &print_node_handle.input_port_keys[0] );
    }

    let compile_result = compile( &mut project, &CompileSettings::new() ).unwrap();
    let program = compile_result.program;

    let compiled_graph = program.compiled_graphs.get(&project.entry_graph).unwrap();

    assert_eq!(compiled_graph.instructions[0], Instruction::SetConst(0, Value::Float(1.0)));
    assert_eq!(compiled_graph.instructions[1], Instruction::Wait(0));
    assert_eq!(compiled_graph.instructions[2], Instruction::SetConst(1, Value::Integer(0)));
    assert_eq!(compiled_graph.instructions[3], Instruction::Print(1));
    assert_eq!(compiled_graph.instructions[4], Instruction::Jump(0));
    assert_eq!(compiled_graph.instructions[5], Instruction::Return);
}
