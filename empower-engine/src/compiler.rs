use std::collections::VecDeque;
use std::collections::HashMap;

use crate::assets::AssetId;
use crate::node_graph::NodeGraphKey;
use crate::node_graph::Port;
use crate::node_graph::port;
use crate::project::Project;

mod debugger;
use debugger::DebugSettings;

mod instructions;
pub use instructions::Instruction;
pub use instructions::InstructionSet;

mod compiler_context;
pub use compiler_context::CompilerContext;

#[derive(Clone)]
pub struct Program
{
    pub entry_graph_id: AssetId,
    pub compiled_graphs: HashMap<AssetId, CompiledGraph>,
}

pub fn debug_compile(project: &Project, debug_settings: &DebugSettings)
{
    
}

pub fn release_compile(project: &Project) -> Result<Program, &'static str>
{
    let mut ctx = CompilerContext::new_with_graph_to_compile(project.entry_graph);

    loop
    {
        let next_graph_to_compile = ctx.get_next_graph_to_compile();

        if next_graph_to_compile.is_none()
        {
            break;
        }

        let next_graph_to_compile =next_graph_to_compile.unwrap();
        
        compile_graph(&mut ctx, next_graph_to_compile, project);
    }

    Ok( ctx.get_program(project.entry_graph) )
}

pub fn compile_graph(ctx: &mut CompilerContext, graph_id: AssetId, project: &Project)
{
    ctx.begin_compiling_new_graph();

    let graph = project.assets.get_node_graph(&graph_id).unwrap(); 

    let mut next_nodes_to_compile = VecDeque::from(graph.input_nodes.clone());

    let mut registered_ports = HashMap::new();

    let inverted_connections = invert_connections(&graph.connections);

    while !next_nodes_to_compile.is_empty()
    {
        let node_to_compile = next_nodes_to_compile.pop_front().unwrap();

        let (node, inputs, outputs) = graph.get_node_input_output(node_to_compile).expect("unable to compile node as something is wrong in node");  // @TODO, take a second look at this function

        let input_slots = preallocate_registers(&inputs, &mut registered_ports, &graph.connections, ctx);
        let output_slots = allocate_registers(&outputs, &mut registered_ports, ctx);

        node.kind.compile(ctx, input_slots, output_slots);

        let connected_nodes_to_compile = get_connected_exec_nodes(outputs, &inverted_connections);
        next_nodes_to_compile.extend(connected_nodes_to_compile);
    }

    ctx.finish_compiling_current_graph(graph_id);
}

fn allocate_registers(ports: &Vec<&Port>, registered_ports: &mut HashMap<NodeGraphKey, RegisterAddress>, ctx: &mut CompilerContext) -> Vec<RegisterAddress>
{
    let compiling_graph = ctx.get_current_compiling_graph();
    
    let mut registers = Vec::new();

    for port in ports
    {
        match port.kind
        {
            port::PortKind::Execution => { continue; },
            port::PortKind::Data => {},
        }

        registers.push(compiling_graph.register_size);
        compiling_graph.register_size += 1;

        registered_ports.insert(port.key, compiling_graph.register_size);
    }

    registers
}

fn preallocate_registers(ports: &Vec<&Port>, registered_ports: &mut HashMap<NodeGraphKey, RegisterAddress>, connections: &HashMap<NodeGraphKey, NodeGraphKey>, ctx: &mut CompilerContext) -> Vec<RegisterAddress>
{
    let compiling_graph = ctx.get_current_compiling_graph();

    let mut registers = Vec::new();

    for port in ports
    {
        match port.kind
        {
            port::PortKind::Execution => { continue; },
            port::PortKind::Data => {},
        }

        if !connections.contains_key(&port.key)
        {
            registers.push(compiling_graph.register_size);
            compiling_graph.instructions.push( Instruction::SetConst(compiling_graph.register_size, port.get_value().unwrap()));

            compiling_graph.register_size += 1;

            registered_ports.insert(port.node_key, compiling_graph.register_size);

            continue;
        }

        let connected_port = connections.get(&port.key).unwrap();
        let register = registered_ports.get(connected_port).unwrap();

        registers.push(*register);
    }

    registers
}

#[derive(Clone)]
pub struct CompiledGraph
{
    pub register_size: i32, 
    pub instructions: Vec<Instruction>,
}

impl CompiledGraph
{
    pub fn new() -> Self
    {
        Self
        {
            register_size: 0,
            instructions: Vec::new(),
        }
    }
}

pub type RegisterAddress = i32;

fn invert_connections(connections: &HashMap<NodeGraphKey, NodeGraphKey>) -> HashMap<i32, Vec<i32>>
{
    let mut inverted_connections: HashMap<i32, Vec<i32>> = HashMap::new();

    for (&key, &value) in connections
    {
        inverted_connections.entry(value).or_default().push(key);
    }
    
    inverted_connections
}

fn get_connected_exec_nodes(ports: Vec<&Port>, inverted_connections: &HashMap<NodeGraphKey, Vec<NodeGraphKey>>) -> Vec<NodeGraphKey>
{
    if ports.is_empty()
    {
        return Vec::new();
    }

    let exec_port = ports[0];

    match exec_port.kind
    {
        port::PortKind::Execution => {},
        port::PortKind::Data => { return Vec::new(); },
    }

    let connections = inverted_connections.get(&exec_port.key);

    if connections.is_none()
    {
        return Vec::new();
    }

    connections.unwrap().clone()
}
