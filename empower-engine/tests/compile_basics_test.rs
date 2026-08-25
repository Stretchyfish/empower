use empower_engine::{self, compiler::{CompileSettings, Instruction, compile}, node_graph::node::{NodeKind, node_kind::LoopMode}, value::Value};

#[test]
fn test_print_compile()
{
    let mut project = empower_engine::project::Project::new();

    {
        let entry_node_graph = project.assets.get_node_graph_mut(&project.entry_graph).unwrap();

        let start_node_handle = entry_node_graph.get_node_handle(entry_node_graph.start_node_key);
        let print_node_handle = entry_node_graph.add_node(NodeKind::Print, None);

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
        let branch_node_handle = entry_node_graph.add_node(NodeKind::Branch, None);

        let _ = entry_node_graph.add_connection(&start_node_handle.output_port_keys[0], &branch_node_handle.input_port_keys[0] );
    }

    let compile_result = compile( &mut project, &CompileSettings::new() ).unwrap();
    let program = compile_result.program;

    let compiled_graph = program.compiled_graphs.get(&project.entry_graph).unwrap();

    assert_eq!(compiled_graph.instructions[0], Instruction::SetConst(0, Value::Bool(false)));
    assert_eq!(compiled_graph.instructions[1], Instruction::JumpIfFalse(3, 0));
    assert_eq!(compiled_graph.instructions[2], Instruction::Jump(3));
    assert_eq!(compiled_graph.instructions[3], Instruction::Return);
}

#[test]
fn test_loop_forever_compile()
{
    let mut project = empower_engine::project::Project::new();

    {
        let entry_node_graph = project.assets.get_node_graph_mut(&project.entry_graph).unwrap();

        let start_node_handle = entry_node_graph.get_node_handle(entry_node_graph.start_node_key);
        let loop_node_handle = entry_node_graph.add_node(NodeKind::Loop(LoopMode::Forever), None);

        let _ = entry_node_graph.add_connection(&start_node_handle.output_port_keys[0], &loop_node_handle.input_port_keys[0] );
    }

    let compile_result = compile( &mut project, &CompileSettings::new() ).unwrap();
    let program = compile_result.program;

    let compiled_graph = program.compiled_graphs.get(&project.entry_graph).unwrap();

    assert_eq!(compiled_graph.instructions[0], Instruction::Jump(0));
    assert_eq!(compiled_graph.instructions[1], Instruction::Return);
}

#[test]
fn test_loop_ranged_compile()
{
    let mut project = empower_engine::project::Project::new();

    {
        let entry_node_graph = project.assets.get_node_graph_mut(&project.entry_graph).unwrap();

        let start_node_handle = entry_node_graph.get_node_handle(entry_node_graph.start_node_key);
        let loop_node_handle = entry_node_graph.add_node(NodeKind::Loop(LoopMode::Range), None);

        let _ = entry_node_graph.add_connection(&start_node_handle.output_port_keys[0], &loop_node_handle.input_port_keys[0] );
    }

    let compile_result = compile( &mut project, &CompileSettings::new() ).unwrap();
    let program = compile_result.program;

    let compiled_graph = program.compiled_graphs.get(&project.entry_graph).unwrap();

    assert_eq!(compiled_graph.instructions[0], Instruction::SetConst(0, Value::Integer(0)));
    assert_eq!(compiled_graph.instructions[1], Instruction::SetConst(1, Value::Integer(1)));
    assert_eq!(compiled_graph.instructions[2], Instruction::SetConst(2, Value::Integer(10)));
    assert_eq!(compiled_graph.instructions[3], Instruction::Copy(0, 4));
    assert_eq!(compiled_graph.instructions[4], Instruction::Compare(2, 4, 3));
    assert_eq!(compiled_graph.instructions[5], Instruction::JumpIfTrue(8, 3));
    assert_eq!(compiled_graph.instructions[6], Instruction::Add(4, 1, 4));
    assert_eq!(compiled_graph.instructions[7], Instruction::Jump(4));
    assert_eq!(compiled_graph.instructions[8], Instruction::Return);
}
