use empower_engine::{self, assets::{ASSET_FOLDER_ASSET_ID, AssetKind}, compiler::{CompileSettings, Instruction, compile}, node_graph::node::{NodeKind, node_kind::{ImageState, ListState, LoopMode, ReadFileState, SubGraphState}}, value::Value};

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
fn test_nested_branch_compile()
{
    let mut project = empower_engine::project::Project::new();

    {
        let graph = project.assets.get_node_graph_mut(&project.entry_graph).unwrap();
        let start = graph.get_node_handle(graph.start_node_key);
        let outer = graph.add_node(NodeKind::Branch, None);
        let inner = graph.add_node(NodeKind::Branch, None);

        graph.add_connection(&start.output_port_keys[0], &outer.input_port_keys[0]).unwrap();
        graph.add_connection(&outer.output_port_keys[0], &inner.input_port_keys[0]).unwrap();
    }

    let result = compile(&mut project, &CompileSettings::new()).unwrap();
    let instructions = &result.program.compiled_graphs[&project.entry_graph].instructions;

    assert_eq!(instructions[1], Instruction::JumpIfFalse(6, 0));
    assert_eq!(instructions[3], Instruction::JumpIfFalse(5, 1));
    assert_eq!(instructions[4], Instruction::Jump(5));
    assert_eq!(instructions[5], Instruction::Jump(6));
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

#[test]
fn test_subgraph_compile()
{
    let mut project = empower_engine::project::Project::new();

    let function_asset_creation_result = project.assets.create_asset(None, AssetKind::NodeGraph, "function");
    let function_graph_id = function_asset_creation_result.unwrap();

    {
        let subgraph_node = 
        {
            let function_graph = project.assets.get_node_graph_mut(&function_graph_id).unwrap();
            let print_node_handle = function_graph.add_node(NodeKind::Print, None);

            let start_node_handle = function_graph.get_node_handle(function_graph.start_node_key);
            let _ = function_graph .add_connection(&start_node_handle.output_port_keys[0], &print_node_handle.input_port_keys[0] );

            NodeKind::SubGraph( SubGraphState::from( function_graph_id, function_graph ) )             
        };

        let entry_node_graph = project.assets.get_node_graph_mut(&project.entry_graph).unwrap();

        let start_node_handle = entry_node_graph.get_node_handle(entry_node_graph.start_node_key);
        let subgraph_node_handle = entry_node_graph.add_node( subgraph_node, None);

        // @TODO, check the results 
        let _ = entry_node_graph.add_connection(&start_node_handle.output_port_keys[0], &subgraph_node_handle.input_port_keys[0] );
    }

    let compile_result = compile( &mut project, &CompileSettings::new() ).unwrap();
    let program = compile_result.program;

    let compiled_entry_graph = program.compiled_graphs.get(&project.entry_graph).unwrap();
    let compiled_function_graph = program.compiled_graphs.get(&function_graph_id).unwrap();

    assert_eq!(compiled_entry_graph.instructions[0], Instruction::CallGraph(function_graph_id));
    assert_eq!(compiled_entry_graph.instructions[1], Instruction::Return);

    assert_eq!(compiled_function_graph.instructions[0], Instruction::SetConst(0, Value::Integer(0)));
    assert_eq!(compiled_function_graph.instructions[1], Instruction::Print(0));
    assert_eq!(compiled_function_graph.instructions[2], Instruction::Return);
}

#[test]
fn test_show_image_compile()
{
    let mut project = empower_engine::project::Project::new();

    {
        let entry_node_graph = project.assets.get_node_graph_mut(&project.entry_graph).unwrap();

        let start_node_handle = entry_node_graph.get_node_handle(entry_node_graph.start_node_key);
        let show_image_handle = entry_node_graph.add_node(NodeKind::ShowImage, None);

        let _ = entry_node_graph.add_connection(&start_node_handle.output_port_keys[0], &show_image_handle.input_port_keys[0] );
    }

    let compile_result = compile( &mut project, &CompileSettings::new() ).unwrap();
    let program = compile_result.program;

    let compiled_graph = program.compiled_graphs.get(&project.entry_graph).unwrap();

    assert_eq!(compiled_graph.instructions[0], Instruction::SetConst(0, Value::Image(None)));
    assert_eq!(compiled_graph.instructions[1], Instruction::ShowImage(0));
    assert_eq!(compiled_graph.instructions[2], Instruction::Return );
}

#[test]
fn test_image_feeds_show_image_compile()
{
    let mut project = empower_engine::project::Project::new();
    let image_asset_id = 42;

    {
        let graph = project.assets.get_node_graph_mut(&project.entry_graph).unwrap();
        let start = graph.get_node_handle(graph.start_node_key);
        let image = graph.add_node(NodeKind::Image(ImageState::from(image_asset_id)), None);
        let show_image = graph.add_node(NodeKind::ShowImage, None);

        graph.add_connection(&start.output_port_keys[0], &show_image.input_port_keys[0]).unwrap();
        graph.add_connection(&image.output_port_keys[0], &show_image.input_port_keys[1]).unwrap();
    }

    let result = compile(&mut project, &CompileSettings::new()).unwrap();
    let instructions = &result.program.compiled_graphs[&project.entry_graph].instructions;

    assert_eq!(instructions[0], Instruction::SetConst(0, Value::Image(Some(image_asset_id))));
    assert_eq!(instructions[1], Instruction::ShowImage(0));
    assert_eq!(instructions[2], Instruction::Return);
}

#[test]
fn test_math_graph_compile()
{
    let mut project = empower_engine::project::Project::new();

    {
        let entry_node_graph = project.assets.get_node_graph_mut(&project.entry_graph).unwrap();

        let start_node_handle = entry_node_graph.get_node_handle(entry_node_graph.start_node_key);
        let math_graph_handle = entry_node_graph.add_node(NodeKind::MathGraph, None);

        let _ = entry_node_graph.add_connection(&start_node_handle.output_port_keys[0], &math_graph_handle.input_port_keys[0] );
    }

    let compile_result = compile( &mut project, &CompileSettings::new() ).unwrap();
    let program = compile_result.program;

    let compiled_graph = program.compiled_graphs.get(&project.entry_graph).unwrap();

    assert_eq!(compiled_graph.instructions[0], Instruction::SetConst(0, Value::List(Vec::new())));
    assert_eq!(compiled_graph.instructions[1], Instruction::ShowMathGraph(0));
    assert_eq!(compiled_graph.instructions[2], Instruction::Return );
}

#[test]
fn test_list_feeds_math_graph_compile()
{
    let mut project = empower_engine::project::Project::new();

    {
        let graph = project.assets.get_node_graph_mut(&project.entry_graph).unwrap();
        let start = graph.get_node_handle(graph.start_node_key);
        let list = graph.add_node(NodeKind::List(ListState::new()), None);
        let math_graph = graph.add_node(NodeKind::MathGraph, None);

        graph.add_connection(&start.output_port_keys[0], &math_graph.input_port_keys[0]).unwrap();
        graph.add_connection(&list.output_port_keys[0], &math_graph.input_port_keys[1]).unwrap();
    }

    let result = compile(&mut project, &CompileSettings::new()).unwrap();
    let instructions = &result.program.compiled_graphs[&project.entry_graph].instructions;

    assert_eq!(instructions[0], Instruction::SetConst(0, Value::Integer(0)));
    assert_eq!(instructions[1], Instruction::SetConst(1, Value::Integer(0)));
    assert_eq!(instructions[2], Instruction::CreateList(vec![0, 1], 2));
    assert_eq!(instructions[3], Instruction::ShowMathGraph(2));
    assert_eq!(instructions[4], Instruction::Return);
}

#[test]
fn test_read_file_as_asset_compile()
{
    let mut project = empower_engine::project::Project::new();

    let asset_id = project.assets.create_asset(Some( ASSET_FOLDER_ASSET_ID ), AssetKind::Json, "test");

    let Ok(asset_id) = asset_id else { panic!("unable to create json asset"); };

    {
        let entry_node_graph = project.assets.get_node_graph_mut(&project.entry_graph).unwrap();

        let start_node_handle = entry_node_graph.get_node_handle(entry_node_graph.start_node_key);
        let read_file_handle = entry_node_graph.add_node( NodeKind::ReadFile( ReadFileState::Asset( Some( asset_id ))), None );

        let _ = entry_node_graph.add_connection(&start_node_handle.output_port_keys[0], &read_file_handle .input_port_keys[0] );
    }

    let compile_result = compile( &mut project, &CompileSettings::new() ).unwrap();
    let program = compile_result.program;

    let compiled_graph = program.compiled_graphs.get(&project.entry_graph).unwrap();

    assert_eq!(compiled_graph.instructions[0], Instruction::ReadAsset(0, asset_id));
    assert_eq!(compiled_graph.instructions[1], Instruction::Return );
}
