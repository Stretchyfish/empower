use empower_engine::executor::{Executor, ExecutorSettings};
use empower_engine::node_graph::node::node_kind::LoopMode;
use empower_engine::{self, compiler::{CompileSettings, compile}, node_graph::node::NodeKind, value::Value};

#[test]
fn print_executed_test()
{
    let mut project = empower_engine::project::Project::new();

    {
        let entry_node_graph = project.assets.get_node_graph_mut(&project.entry_graph).unwrap();

        let start_node_handle = entry_node_graph.get_node_handle(entry_node_graph.start_node_key);
        let print_node_handle = entry_node_graph.add_node(NodeKind::Print, None);

        // @TODO, check the results 
        let _ = entry_node_graph.add_connection(&start_node_handle.output_port_keys[0], &print_node_handle.input_port_keys[0] );
    }

    let compile_result = compile( &mut project, &CompileSettings::new() ).unwrap();
    let program = compile_result.program;

    let mut executor = Executor::new(program, ExecutorSettings::new());

    let mut outputs = Vec::new();

    while executor.is_running()
    {
        let executor_outputs = executor.run(None);
        outputs.extend(executor_outputs.outputs);
    }

    assert_eq!(outputs[0], 0.to_string());
}

#[test]
fn true_branch_executed_test()
{
    let mut project = empower_engine::project::Project::new();

    {
        let entry_node_graph = project.assets.get_node_graph_mut(&project.entry_graph).unwrap();

        let start_node_handle = entry_node_graph.get_node_handle(entry_node_graph.start_node_key);
        let branch_node_handle = entry_node_graph.add_node(NodeKind::Branch, None);
        let print_node_1_handle = entry_node_graph.add_node(NodeKind::Print, None);
        let print_node_2_handle = entry_node_graph.add_node(NodeKind::Print, None);

        // @TODO, check the results 
        let _ = entry_node_graph.add_connection(&start_node_handle.output_port_keys[0], &branch_node_handle.input_port_keys[0] );
        let _ = entry_node_graph.add_connection(&branch_node_handle.output_port_keys[0], &print_node_1_handle.input_port_keys[0] );
        let _ = entry_node_graph.add_connection(&branch_node_handle.output_port_keys[1], &print_node_2_handle.input_port_keys[0] );

        let _ = entry_node_graph.set_port_value(branch_node_handle.input_port_keys[1], Value::Bool(true) );
        let _ = entry_node_graph.set_port_value(print_node_1_handle.input_port_keys[1], Value::Integer(1) );
        let _ = entry_node_graph.set_port_value(print_node_2_handle.input_port_keys[1], Value::Integer(2) );
    }

    let compile_result = compile( &mut project, &CompileSettings::new() ).unwrap();
    let program = compile_result.program;

    let _ = program.compiled_graphs.get(&project.entry_graph).unwrap();

    let mut executor = Executor::new(program, ExecutorSettings::new());

    let mut output = Vec::new();

    while executor.is_running()
    {
        let executor_outputs = executor.run(None);
        output.extend(executor_outputs.outputs);
    }

    assert_eq!(output[0], 1.to_string()); // True branch output
}

#[test]
fn false_branch_executed_test()
{
    let mut project = empower_engine::project::Project::new();

    {
        let entry_node_graph = project.assets.get_node_graph_mut(&project.entry_graph).unwrap();

        let start_node_handle = entry_node_graph.get_node_handle(entry_node_graph.start_node_key);
        let branch_node_handle = entry_node_graph.add_node(NodeKind::Branch, None);
        let print_node_1_handle = entry_node_graph.add_node(NodeKind::Print, None);
        let print_node_2_handle = entry_node_graph.add_node(NodeKind::Print, None);

        // @TODO, check the results 
        let _ = entry_node_graph.add_connection(&start_node_handle.output_port_keys[0], &branch_node_handle.input_port_keys[0] );
        let _ = entry_node_graph.add_connection(&branch_node_handle.output_port_keys[0], &print_node_1_handle.input_port_keys[0] );
        let _ = entry_node_graph.add_connection(&branch_node_handle.output_port_keys[1], &print_node_2_handle.input_port_keys[0] );

        let _ = entry_node_graph.set_port_value(branch_node_handle.input_port_keys[1], Value::Bool(false) );
        let _ = entry_node_graph.set_port_value(print_node_1_handle.input_port_keys[1], Value::Integer(1) );
        let _ = entry_node_graph.set_port_value(print_node_2_handle.input_port_keys[1], Value::Integer(2) );
    }

    let compile_result = compile( &mut project, &CompileSettings::new() ).unwrap();
    let program = compile_result.program;

    let _ = program.compiled_graphs.get(&project.entry_graph).unwrap();

    let mut executor = Executor::new(program, ExecutorSettings::new());

    let mut output = Vec::new();
    while executor.is_running()
    {
        let executor_outputs = executor.run(None);
        output.extend(executor_outputs.outputs);
    }

    assert_eq!(output[0], 2.to_string()); // False branch output
}

#[test]
fn forever_loop_execution_test()
{
    let mut project = empower_engine::project::Project::new();

    {
        let entry_node_graph = project.assets.get_node_graph_mut(&project.entry_graph).unwrap();

        let start_node_handle = entry_node_graph.get_node_handle(entry_node_graph.start_node_key);
        let loop_node_handle = entry_node_graph.add_node(NodeKind::Loop( LoopMode::Forever ), None);
        let print_node_handle = entry_node_graph.add_node(NodeKind::Print, None);

        // @TODO, check the results 
        let _ = entry_node_graph.add_connection(&start_node_handle.output_port_keys[0], &loop_node_handle.input_port_keys[0] );
        let _ = entry_node_graph.add_connection(&loop_node_handle.output_port_keys[0], &print_node_handle.input_port_keys[0] );
    }

    let compile_result = compile( &mut project, &CompileSettings::new() ).unwrap();
    let program = compile_result.program;

    let _ = program.compiled_graphs.get(&project.entry_graph).unwrap();

    let mut executor = Executor::new(program, ExecutorSettings::new());

    let mut output = Vec::new();
    for _ in 0..30 // Limited in runs due to running "forever" (SetConst, Print, Jump, Return), 3x10 instructions
    {
        let executor_outputs = executor.run(None);
        output.extend(executor_outputs.outputs);
    }

    assert_eq!(output[0], 0.to_string()); // Testing that there is 10 outputs (could be written better)
    assert_eq!(output[1], 0.to_string()); 
    assert_eq!(output[2], 0.to_string()); 
    assert_eq!(output[3], 0.to_string()); 
    assert_eq!(output[4], 0.to_string()); 
    assert_eq!(output[5], 0.to_string()); 
    assert_eq!(output[6], 0.to_string()); 
    assert_eq!(output[7], 0.to_string()); 
    assert_eq!(output[8], 0.to_string()); 
    assert_eq!(output[9], 0.to_string()); 
}

#[test]
fn ranged_loop_execution_test()
{
    let mut project = empower_engine::project::Project::new();

    {
        let entry_node_graph = project.assets.get_node_graph_mut(&project.entry_graph).unwrap();

        let start_node_handle = entry_node_graph.get_node_handle(entry_node_graph.start_node_key);
        let loop_node_handle = entry_node_graph.add_node(NodeKind::Loop( LoopMode::Range ), None);
        let print_node_handle = entry_node_graph.add_node(NodeKind::Print, None);

        // @TODO, check the results 
        let _ = entry_node_graph.add_connection(&start_node_handle.output_port_keys[0], &loop_node_handle.input_port_keys[0] );
        let _ = entry_node_graph.add_connection(&loop_node_handle.output_port_keys[0], &print_node_handle.input_port_keys[0] );
        let _ = entry_node_graph.add_connection(&loop_node_handle.output_port_keys[2], &print_node_handle.input_port_keys[1] );
    }

    let compile_result = compile( &mut project, &CompileSettings::new() ).unwrap();
    let program = compile_result.program;

    let _ = program.compiled_graphs.get(&project.entry_graph).unwrap();

    let mut executor = Executor::new(program, ExecutorSettings::new());

    let mut output = Vec::new();
    while executor.is_running()
    {
        let executor_outputs = executor.run(None);
        output.extend(executor_outputs.outputs);
    }

    assert_eq!(output[0], 0.to_string()); // Testing that there is 10 outputs (could be written better)
    assert_eq!(output[1], 1.to_string()); 
    assert_eq!(output[2], 2.to_string()); 
    assert_eq!(output[3], 3.to_string()); 
    assert_eq!(output[4], 4.to_string()); 
    assert_eq!(output[5], 5.to_string()); 
    assert_eq!(output[6], 6.to_string()); 
    assert_eq!(output[7], 7.to_string()); 
    assert_eq!(output[8], 8.to_string()); 
    assert_eq!(output[9], 9.to_string()); 
}
