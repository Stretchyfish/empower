use std::collections::VecDeque;
use std::collections::HashMap;
use std::collections::HashSet;

use crate::assets::AssetId;
use crate::node_graph::NodeGraph;
use crate::node_graph::NodeGraphKey;
use crate::node_graph::Port;
use crate::node_graph::port;
use crate::node_graph::port::PortKind;
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
    let graph = project.assets.get_node_graph(&graph_id).unwrap(); 
    let mut compiled_graph_context = CompiledGraphContext::new(graph_id, graph.input_nodes.clone());

    let mut next_nodes_to_compile = VecDeque::from(graph.input_nodes.clone());

    while !next_nodes_to_compile.is_empty()
    {
        let node_to_compile = next_nodes_to_compile.front().unwrap();

        let other_node_to_compile_first = check_if_node_needs_another_node_compiled_first(&mut compiled_graph_context, node_to_compile, graph);

        if other_node_to_compile_first.is_some()
        {
            next_nodes_to_compile.push_front(other_node_to_compile_first.unwrap());
            continue;
        }

        let connected_exec_ports;

        {
            let (node, inputs, outputs) = graph.get_node_input_output(*node_to_compile).expect("unable to compile node as something is wrong in node");  // @TODO, take a second look at this function

            let input_slots = allocate_input_port_registers(&mut compiled_graph_context, &inputs, &graph.connections_in);
            let output_slots = allocate_output_port_registers(&mut compiled_graph_context, &outputs);

            node.kind.compile(&mut compiled_graph_context, input_slots, output_slots);

            connected_exec_ports = get_connected_exec_ports(outputs, &graph.connections_out);
        }
        
        next_nodes_to_compile.extend( get_next_nodes_to_compile(&connected_exec_ports, graph) );
        next_nodes_to_compile.pop_front();
    }

    compiled_graph_context.add_instruction( Instruction::Return );

    ctx.add_more_graphs_to_build(&compiled_graph_context.additional_graphs_to_compile);
    ctx.add_compiled_graph(graph_id, CompiledGraph::from_compiled_graph_context(compiled_graph_context));
}

fn check_if_node_needs_another_node_compiled_first(ctx: &mut CompiledGraphContext, node_key: &NodeGraphKey, node_graph: &NodeGraph) -> Option<NodeGraphKey>
{
    let input_port_keys = &node_graph.nodes.get(node_key).unwrap().input_port_keys;

    for input_port_key in input_port_keys
    {
        if !node_graph.connections_in.contains_key(input_port_key)
        {
            continue;
        }

        match node_graph.ports.get(input_port_key).unwrap().kind
        {
            PortKind::Execution => { continue; },
            PortKind::Data => {},
        }

        let connected_output_port_key = node_graph.connections_in.get(input_port_key).unwrap();

        if ctx.allocated_ports.contains_key(connected_output_port_key)
        {
            continue;
        }

        let output_port = node_graph.ports.get( connected_output_port_key ).unwrap();
        
        return Some( output_port.node_key );
    }

    None
}

fn allocate_input_port_registers(ctx: &mut CompiledGraphContext, ports: &Vec<&Port>, connections_in: &HashMap<NodeGraphKey, NodeGraphKey>) -> Vec<RegisterAddress>
{
    let mut input_registers = Vec::with_capacity(ports.len());

    for port in ports
    {
        match port.kind
        {
            port::PortKind::Execution => { continue; },
            port::PortKind::Data => {},
        }

        if !connections_in.contains_key(&port.key)
        {
            ctx.instructions.push( Instruction::SetConst(ctx.register_size, port.value.as_ref().unwrap().clone()));

            ctx.allocated_ports.insert(port.key, ctx.register_size);
            input_registers.push(ctx.register_size);

            ctx.register_size += 1;

            continue;
        }

        let connected_output_port = connections_in.get(&port.key).unwrap();

        // here should be the connection check
        
        let already_allocate_register_address = ctx.allocated_ports.get(connected_output_port).unwrap();

        input_registers.push(*already_allocate_register_address);
    }

    println!("Registers added {:?}", input_registers);

    input_registers
}

fn allocate_output_port_registers(ctx: &mut CompiledGraphContext, ports: &Vec<&Port>) -> Vec<RegisterAddress>
{
    let mut output_registers = Vec::with_capacity(ports.len());

    for port in ports
    {
        match port.kind
        {
            port::PortKind::Execution => { continue; },
            port::PortKind::Data => {},
        }

        ctx.allocated_ports.insert(port.key, ctx.register_size);
        output_registers.push(ctx.register_size);

        ctx.register_size += 1;
    }

    output_registers
}

#[derive(Clone)]
pub struct CompiledGraphContext
{
    pub graph_id: AssetId,
    pub nodes_to_compile_queue: VecDeque<NodeGraphKey>,
    pub allocated_ports: HashMap<NodeGraphKey, RegisterAddress>,
    
    pub register_size: i32, 
    pub instructions: Vec<Instruction>,

    pub additional_graphs_to_compile: Vec<AssetId>,
}

impl CompiledGraphContext
{
    pub fn new(graph_id: AssetId, start_nodes: Vec<NodeGraphKey>) -> Self
    {
        Self
        {
            graph_id,
            nodes_to_compile_queue: VecDeque::from(start_nodes),
            allocated_ports: HashMap::new(),

            register_size: 0,
            instructions: Vec::new(),

            additional_graphs_to_compile: Vec::new(),
        }
    }

    pub fn add_instruction(&mut self, instruction: Instruction)
    {
        self.instructions.push(instruction);
    }
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

    pub fn from_compiled_graph_context(ctx: CompiledGraphContext) -> Self
    {
        Self
        {
            register_size: ctx.register_size,
            instructions: ctx.instructions,
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

fn get_connected_exec_ports(ports: Vec<&Port>, connections_out: &HashMap<NodeGraphKey, HashSet<NodeGraphKey>>) -> HashSet<NodeGraphKey>
{
    if ports.is_empty()
    {
        return HashSet::new();
    }

    let exec_port = ports[0];

    match exec_port.kind
    {
        port::PortKind::Execution => {},
        port::PortKind::Data => { return HashSet::new(); },
    }

    let connections = connections_out.get(&exec_port.key);

    if connections.is_none()
    {
        return HashSet::new();
    }

    connections.unwrap().clone()
}

fn get_next_nodes_to_compile(connected_exec_ports: &HashSet<NodeGraphKey>, node_graph: &NodeGraph) -> Vec<NodeGraphKey>
{
    let mut connected_nodes = Vec::with_capacity(connected_exec_ports.len());

    for connected_exec_port in connected_exec_ports
    {
        let node_containing_port = node_graph.ports.get(connected_exec_port).unwrap().node_key;

        connected_nodes.push(node_containing_port);
    }

    connected_nodes
}
