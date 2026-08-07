use empower_engine::{self, compiler::{CompileSettings, Instruction, compile}, node_graph::node::NodeKind2, value::Value};

#[test]
fn test_print_compile()
{
    let mut project = empower_engine::project::Project::new();

    {
        let entry_node_graph = project.assets.get_node_graph_mut(&project.entry_graph).unwrap();

        let start_node_handle = entry_node_graph.get_node_handle(entry_node_graph.start_node_key);
        let print_node_handle = entry_node_graph.add_node(NodeKind2::Print, None);

        let _ = entry_node_graph.add_connection(&start_node_handle.output_port_keys[0], &print_node_handle.input_port_keys[0] );
    }

    let compile_result = compile( &mut project, &CompileSettings::new() ).unwrap();
    let program = compile_result.program;

    let compiled_graph = program.compiled_graphs.get(&project.entry_graph).unwrap();

    assert_eq!(compiled_graph.instructions[0], Instruction::SetConst(0, Value::Integer(0)));
    assert_eq!(compiled_graph.instructions[1], Instruction::Print(0));
    assert_eq!(compiled_graph.instructions[2], Instruction::Return);
}

#[test]
fn test_branch_compile()
{
    let mut project = empower_engine::project::Project::new();

    {
        let entry_node_graph = project.assets.get_node_graph_mut(&project.entry_graph).unwrap();

        let start_node_handle = entry_node_graph.get_node_handle(entry_node_graph.start_node_key);
        let print_node_handle = entry_node_graph.add_node(NodeKind2::Branch, None);

        let _ = entry_node_graph.add_connection(&start_node_handle.output_port_keys[0], &print_node_handle.input_port_keys[0] );
    }

    let compile_result = compile( &mut project, &CompileSettings::new() ).unwrap();
    let program = compile_result.program;

    let compiled_graph = program.compiled_graphs.get(&project.entry_graph).unwrap();

    assert_eq!(compiled_graph.instructions[0], Instruction::SetConst(0, Value::Bool(false)));
    assert_eq!(compiled_graph.instructions[1], Instruction::JumpIfFalse(3, 0));
    assert_eq!(compiled_graph.instructions[2], Instruction::Jump(3));
    assert_eq!(compiled_graph.instructions[3], Instruction::Return);
}
