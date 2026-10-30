use std::collections::HashMap;

use crate::assets::AssetId;
use crate::assets::Assets;
use crate::assets::LoadedAssets;
use crate::node_graph::NodeGraph;
use crate::node_graph::NodeGraphKey;
use crate::node_graph::Port;
use crate::node_graph::port;
use crate::node_graph::port::PortKind;
use crate::project::{Project, ProjectState};
use crate::value::Value;

mod compile_meta;
pub use compile_meta::CompileMeta;

mod instructions;
pub use instructions::Instruction;
pub use instructions::InstructionSet;
pub use instructions::InstructionAddress;

mod compiler_context;
pub use compiler_context::CompilerContext;

use serde::{Deserialize, Serialize};

pub type RegisterAddress = i32;

#[derive(Clone, Serialize, Deserialize)]
pub struct Program
{
    pub entry_graph_id: AssetId,
    pub compiled_graphs: HashMap<AssetId, CompiledGraph>,
    pub loaded_assets: LoadedAssets,
}

pub struct ProgramCompileMeta
{
    pub trace: HashMap<InstructionAddress, NodeGraphKey>,
}

impl Program
{
    pub fn from_json(program_json: &String) -> Result<Program, serde_json::Error>
    {
        serde_json::from_str(&program_json)
    }

    pub fn to_json(&self) -> String
    {
        serde_json::to_string_pretty(&self).unwrap()
    }
}

pub struct CompileSettings
{
    traced: bool,
}

impl CompileSettings
{
    pub fn new() -> Self
    {
        Self
        {
            traced: false,
        }
    }
    
    pub fn new_with_meta() -> Self
    {
        Self
        {
            traced: true
        }
    }
}

pub struct CompilePlan // Instructions on how to compile a single node
{
    pub steps: Vec<CompilePlanStep>,
}

pub enum CompilePlanStep
{
    Emit(Instruction), // Add an instruction to the compiled graph
    CompileFromOutput(usize), // Compile nodes connected to the output port index
    QueueGraph(AssetId), // Compile this graph as part of the program
    Mark(&'static str), // Give the next instruction address a name
    Jump(&'static str), // Jump to a named instruction address
    JumpIfFalse(RegisterAddress, &'static str), // Jump to a named instruction address when the register is false
    JumpIfTrue(RegisterAddress, &'static str), // Jump to a named position when the register is true
}

pub struct CompileResult
{
    pub program: Program,
    pub meta: Option<CompileMeta>,
}

pub fn compile(project: &mut Project, settings: &CompileSettings) -> Result<CompileResult, String>
{
    let mut ctx = CompilerContext::new_with_graph_to_compile(project.entry_graph, settings.traced);

    loop
    {
        let next_graph_to_compile = ctx.get_next_graph_to_compile();

        if next_graph_to_compile.is_none()
        {
            break;
        }

        let next_graph_to_compile = next_graph_to_compile.unwrap();
        
        // project.assets.load_asset(&project.location, next_graph_to_compile); // Safety implementation in case it isn't loaded
        let result = compile_graph(&mut ctx, next_graph_to_compile, project);

        if result.is_err()
        {
            return Err( result.err().unwrap() );
        }
    }

    // project.assets.load_assets(&project.location, ctx.get_assets_to_load() );

    Ok(
        CompileResult
        {
            program: ctx.get_program(project.entry_graph, project.assets.loaded_assets.clone() ),
            meta: ctx.get_meta(), // This will be a none if not defined
        }
    )
}

pub fn compile_graph(ctx: &mut CompilerContext, graph_id: AssetId, project: &Project) -> Result<(), String>
{
    let node_graph = project.assets.get_node_graph(&graph_id).unwrap();
    let assets = &project.assets;

    check_node_graph_compile_integrity(node_graph)?;

    ctx.add_assets_to_load( detect_assets_to_preload(node_graph) );

    let mut compiled_graph_context = CompiledGraphContext::new(graph_id);

    let compile_chain_result = compile_node_chain(&mut compiled_graph_context, assets, node_graph, &node_graph.start_node_key, &project.state); // This initiates the recursive process to compile the entire graph

    if compile_chain_result.is_err()
    {
        return Err(compile_chain_result.unwrap_err());
    }

    compiled_graph_context.add_instruction( &node_graph.start_node_key, Instruction::Return ); 

    ctx.add_more_graphs_to_build(&compiled_graph_context.additional_graphs_to_compile);

    let ( compiled_graph, trace ) = ( CompiledGraph::from(compiled_graph_context.register_size, compiled_graph_context.instructions), compiled_graph_context.trace );

    ctx.add_traced_data( graph_id, trace ); // @TODO, improve this
    ctx.add_compiled_graph(graph_id, compiled_graph);

    Ok(())
}

fn compile_node_chain(ctx: &mut CompiledGraphContext, assets: &Assets, node_graph: &NodeGraph, node_key: &NodeGraphKey, project_state: &ProjectState) -> Result<(), String>
{
    let other_node_to_compile_first = check_if_node_needs_another_node_compiled_first(ctx, node_key, &node_graph);

    if other_node_to_compile_first.is_some()
    {
        compile_node_chain(ctx, assets, node_graph, &other_node_to_compile_first.unwrap(), project_state)?;
    }

    let (node, inputs, outputs) = node_graph.get_node_input_output(*node_key).expect("unable to compile node as something is wrong in node");  // @TODO, take a second look at this function

    let input_register_addresses = allocate_input_port_registers(ctx, node_key, &inputs, &node_graph.connections_in);
    let output_register_addresses = allocate_output_port_registers(ctx, &outputs);

    let node_compile_plan = node.kind.compile(&input_register_addresses, &output_register_addresses, assets, project_state)?;

    let mut labels: HashMap<&'static str, InstructionAddress> = HashMap::new();
    let mut pending_jumps: HashMap<&'static str, Vec<InstructionAddress>> = HashMap::new();

    for step in node_compile_plan.steps
    {
        match step
        {
            CompilePlanStep::Emit(instruction) => ctx.add_instruction(node_key, instruction),
            CompilePlanStep::CompileFromOutput(port_index) => compile_nodes_connected_to_port(ctx, assets, node_graph, &node.output_port_keys[port_index], node_key, project_state)?,
            CompilePlanStep::QueueGraph(graph_id) => ctx.additional_graphs_to_compile.push(graph_id),
            CompilePlanStep::Mark(label) =>
            {
                if labels.contains_key(&label)
                {
                    return Err(format!("Compile plan defines label '{}' twice", label));
                }

                let instruction_address = ctx.get_next_instruction_address();
                labels.insert(label, instruction_address);

                if let Some(jumps_waiting_for_label) = pending_jumps.remove(&label)
                {
                    for jump_address in jumps_waiting_for_label
                    {
                        ctx.patch_jump_instruction(&jump_address, &instruction_address);
                    }
                }
            },
            CompilePlanStep::Jump(label) =>
            {
                if let Some(&destination_address) = labels.get(&label)
                {
                    ctx.add_instruction(node_key, Instruction::Jump(destination_address));
                }
                else
                {
                    let placeholder_address = ctx.add_instruction_placeholder(node_key, Instruction::Jump(0));
                    pending_jumps.entry(label).or_default().push(placeholder_address);
                }
            },
            CompilePlanStep::JumpIfFalse(register, label) =>
            {
                if let Some(&destination_address) = labels.get(&label)
                {
                    ctx.add_instruction(node_key, Instruction::JumpIfFalse(destination_address, register));
                }
                else
                {
                    let placeholder_address = ctx.add_instruction_placeholder(node_key, Instruction::JumpIfFalse(0, register));
                    pending_jumps.entry(label).or_default().push(placeholder_address);
                }
            },
            CompilePlanStep::JumpIfTrue(register, label) =>
            {
                if let Some(&destination_address) = labels.get(&label)
                {
                    ctx.add_instruction(node_key, Instruction::JumpIfTrue(destination_address, register));
                }
                else
                {
                    let placeholder_address = ctx.add_instruction_placeholder(node_key, Instruction::JumpIfTrue(0, register));
                    pending_jumps.entry(label).or_default().push(placeholder_address);
                }
            },
        }
    }

    if let Some(label) = pending_jumps.keys().next()
    {
        return Err(format!("Compile plan references undefined label '{}'", label));
    }


    Ok(())
}

fn check_node_graph_compile_integrity(node_graph: &NodeGraph) -> Result<(), String>
{
    for (port_key, port) in &node_graph.ports
    {
        match port.kind
        {
            PortKind::Execution => { continue; },
            PortKind::Data => {},
        }
        
        if port.compatability.is_empty() // @TODO, this function might not even be needed anymore, due to new implementation
        {
            return Err(format!("Port ({}), does not contain value values", port_key));
        }
    }

    Ok(())
}

fn detect_assets_to_preload(node_graph: &NodeGraph) -> Vec<AssetId>
{
    let mut asset_to_load = Vec::new();

    for (_, port) in &node_graph.ports
    {
        if port.value.is_none()
        {
            continue;
        }

        match port.value.as_ref().unwrap()
        {
            Value::Image( assset_id ) =>
            {
                if assset_id.is_some()
                {
                    asset_to_load.push( assset_id.unwrap() );
                }
            },
            _ => {},
        }
    }

    asset_to_load
}

fn compile_nodes_connected_to_port(ctx: &mut CompiledGraphContext, assets: &Assets, node_graph: &NodeGraph, port_key: &NodeGraphKey, node_key: &NodeGraphKey, project_state: &ProjectState) -> Result<(), String>
{
    let connected_nodes = get_nodes_connected_to_port(*port_key, node_graph);

    for (index, connected_node_key) in connected_nodes.iter().enumerate()
    {
        if index == connected_nodes.len() - 1
        {
            compile_node_chain(ctx, assets, node_graph, connected_node_key, project_state)?;
            continue;
        }

        ctx.add_instruction( node_key, Instruction::Fork( ctx.instructions.len() + 2 ));
        let jump_instruction_address = ctx.add_instruction_placeholder( node_key, Instruction::Jump( 0) );

        compile_node_chain(ctx, assets, node_graph, connected_node_key, project_state)?;

        ctx.add_instruction( node_key, Instruction::Return );

        ctx.patch_jump_instruction(&jump_instruction_address, &ctx.instructions.len() );
    }

    if connected_nodes.len() > 1
    {
        ctx.add_instruction( node_key, Instruction::Join );
    }

    Ok(())
}

fn get_nodes_connected_to_port(port_key: NodeGraphKey, node_graph: &NodeGraph) -> Vec<NodeGraphKey>
{
    let ports_connected_port = node_graph.connections_out.get(&port_key);

    if ports_connected_port.is_none()
    {
        return Vec::new();
    }

    let ports_connected_port = ports_connected_port.unwrap();

    let mut nodes_connected = Vec::with_capacity(ports_connected_port.len());

    for port in ports_connected_port
    {
        nodes_connected.push( node_graph.ports.get(port).unwrap().node_key );
    }

    nodes_connected
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

fn allocate_input_port_registers(ctx: &mut CompiledGraphContext, node_key: &NodeGraphKey, ports: &Vec<&Port>, connections_in: &HashMap<NodeGraphKey, NodeGraphKey>) -> Vec<RegisterAddress>
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
            // @TODO, need to add a warning here to stop compilation, since it might be the case that a port has no defined value yet

            ctx.add_instruction(node_key, Instruction::SetConst(ctx.register_size, port.value.as_ref().unwrap().clone()));

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
    pub allocated_ports: HashMap<NodeGraphKey, RegisterAddress>,
    
    pub register_size: i32, 
    pub instructions: Vec<Instruction>,

    pub additional_graphs_to_compile: Vec<AssetId>,

    pub trace: HashMap<InstructionAddress, NodeGraphKey>, // @TODO, decide if this should be optional, or not get added by default
}

impl CompiledGraphContext
{
    pub fn new(graph_id: AssetId) -> Self
    {
        Self
        {
            graph_id,
            allocated_ports: HashMap::new(),

            register_size: 0,
            instructions: Vec::new(),

            additional_graphs_to_compile: Vec::new(),

            trace: HashMap::new(),
        }
    }

    pub fn add_instruction(&mut self, node_key: &NodeGraphKey, instruction: Instruction)
    {
        self.instructions.push(instruction);
        self.trace.insert(self.instructions.len() - 1, *node_key);
     }

    pub fn add_instruction_placeholder(&mut self, node_key: &NodeGraphKey, instruction: Instruction) -> InstructionAddress
    {
        let instruction_address = self.instructions.len();
        self.instructions.push(instruction);

        self.trace.insert(instruction_address, *node_key);

        instruction_address
    }

    pub fn patch_jump_instruction(&mut self, patch_instruction_address: &InstructionAddress, new_jump_address: &InstructionAddress)
    {
        match &mut self.instructions[*patch_instruction_address]
        {
            Instruction::Jump( instruction_address ) => { *instruction_address = *new_jump_address },
            Instruction::JumpIfFalse( instruction_address, _) => { *instruction_address = *new_jump_address },
            Instruction::JumpIfTrue( instruction_address, _) => { *instruction_address = *new_jump_address },
            _ => todo!(),
        }
    }

    pub fn get_latest_instruction_address(&self) -> InstructionAddress
    {
        if self.instructions.len() == 0
        {
            return 0;
        }

        self.instructions.len() - 1
    }

    pub fn get_next_instruction_address(&self) -> InstructionAddress
    {
        self.instructions.len()
    }
}

#[derive(Clone, Serialize, Deserialize)]
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

    pub fn from(register_size: i32, instructions: Vec<Instruction>) -> Self
    {
        Self
        {
            register_size,
            instructions,
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

