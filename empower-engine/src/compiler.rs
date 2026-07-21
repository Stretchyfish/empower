use std::collections::HashMap;

use crate::assets::AssetId;
use crate::assets::LoadedAssets;
use crate::compiler::instructions::InstructionAddress;
use crate::node_graph::NodeGraph;
use crate::node_graph::NodeGraphKey;
use crate::node_graph::Port;
use crate::node_graph::node::node_kind::ControlFlowKind;
use crate::node_graph::port;
use crate::node_graph::port::PortKind;
use crate::project::Project;
use crate::value::Value;

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
    pub loaded_assets: LoadedAssets,
}

pub fn debug_compile(_: &Project, _: &DebugSettings)
{
    
}

pub fn release_compile(project: &mut Project) -> Result<Program, String>
{
    let mut ctx = CompilerContext::new_with_graph_to_compile(project.entry_graph);

    loop
    {
        let next_graph_to_compile = ctx.get_next_graph_to_compile();

        if next_graph_to_compile.is_none()
        {
            break;
        }

        let next_graph_to_compile = next_graph_to_compile.unwrap();
        
        project.assets.load_asset(&project.location, next_graph_to_compile); // Safety implementation in case it isn't loaded
        let result = compile_graph(&mut ctx, next_graph_to_compile, project);

        if result.is_err()
        {
            return Err( result.err().unwrap() );
        }
    }

    project.assets.load_assets(&project.location, ctx.get_assets_to_load() );

    Ok( ctx.get_program(project.entry_graph, project.assets.loaded_assets.clone() ) )
}

pub fn compile_graph(ctx: &mut CompilerContext, graph_id: AssetId, project: &Project) -> Result<(), String>
{
    let node_graph = project.assets.get_node_graph_naive(&graph_id).unwrap();

    let integrity = check_node_graph_compile_integrity(node_graph);

    if integrity.is_err()
    {
        return integrity;
    }
    
    ctx.add_assets_to_load( detect_assets_to_preload(node_graph) );

    let mut compiled_graph_context = CompiledGraphContext::new(graph_id);

    compile_node_chain(&mut compiled_graph_context, node_graph, &node_graph.start_node_key); // This initiates the recursive process to compile the entire graph

    compiled_graph_context.add_instruction( Instruction::Return );

    ctx.add_more_graphs_to_build(&compiled_graph_context.additional_graphs_to_compile);
    ctx.add_compiled_graph(graph_id, CompiledGraph::from_compiled_graph_context(compiled_graph_context));

    Ok(())
}

fn compile_node_chain(ctx: &mut CompiledGraphContext, node_graph: &NodeGraph, node_key: &NodeGraphKey)
{
    let next_instruction_address = ctx.instructions.len();
    
    let other_node_to_compile_first = check_if_node_needs_another_node_compiled_first(ctx, node_key, &node_graph);
    println!("compiling: {}, and need to also compile first: {:?}", node_key, other_node_to_compile_first);

    if other_node_to_compile_first.is_some()
    {
        compile_node_chain(ctx, node_graph, &other_node_to_compile_first.unwrap());
    }

    let (output_port_keys, control_flow) = 
    {
        let (node, inputs, outputs) = node_graph.get_node_input_output(*node_key).expect("unable to compile node as something is wrong in node");  // @TODO, take a second look at this function

        let input_slots = allocate_input_port_registers(ctx, &inputs, &node_graph.connections_in);
        let output_slots = allocate_output_port_registers(ctx, &outputs);

        node.kind.compile(ctx, input_slots, output_slots);

        (node.output_port_keys.clone(), node.kind.control_flow())
    };

    if output_port_keys.is_empty() // This is for nodes with no output ports
    {
        return;
    }

    match control_flow
    {
        ControlFlowKind::None => {},
        ControlFlowKind::Linear =>
        {
            compile_nodes_connected_to_port(ctx, node_graph, &output_port_keys[0]);
        },
        ControlFlowKind::Branch =>
        {
            // This implementation right now still adds a few instructions if no nodes at all are connected, could be optimized in the future
            
            let jump_if_false_instruction_placeholder_address = ctx.get_latest_instruction_address(); // Since the branch needs access to input port keys, the actual jump_if_false instrcution is added in the branch node compile function

            compile_nodes_connected_to_port(ctx, node_graph, &output_port_keys[0]);

            let jump_after_true_branch_placeholder_address = ctx.add_instruction_placeholder( Instruction::Jump(0) );
            ctx.patch_jump_instruction(&jump_if_false_instruction_placeholder_address, &ctx.instructions.len());

            compile_nodes_connected_to_port(ctx, node_graph, &output_port_keys[1]);

            ctx.patch_jump_instruction(&jump_after_true_branch_placeholder_address, &ctx.instructions.len());
        },
        ControlFlowKind::Loop =>
        {
            // @TOD, current loop implementation, has bug that when its connected to multiple other nodes, it will continue loop if the main branch finished, and it will not wait for the others to finish
            
            // let first_instruction_in_loop_address = ctx.get_latest_instruction_address() + 1; // this might have issues if there are no next instructions (stay aware of this in the future)
            let first_instruction_in_loop_address = ctx.get_latest_instruction_address(); // this might have issues if there are no next instructions (stay aware of this in the future)
            compile_nodes_connected_to_port(ctx, node_graph, &output_port_keys[0]);

            // ctx.add_instruction( Instruction::Jump( first_instruction_in_loop_address ) );
            ctx.add_instruction( Instruction::Jump( next_instruction_address ) );
        },
    };
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

fn compile_nodes_connected_to_port(ctx: &mut CompiledGraphContext, node_graph: &NodeGraph, port_key: &NodeGraphKey)
{
    let connected_nodes = get_nodes_connected_to_port(*port_key, node_graph);

    for (index, connected_node_key) in connected_nodes.iter().enumerate()
    {
        println!("Crashed here? 1");
        if index == connected_nodes.len() - 1
        {
            compile_node_chain(ctx, node_graph, connected_node_key);
            continue;
        }
        println!("Crashed here? 2");

        ctx.add_instruction( Instruction::Fork( ctx.instructions.len() + 2 ));
        let jump_instruction_address = ctx.add_instruction_placeholder( Instruction::Jump( 0) );

        compile_node_chain(ctx, node_graph, connected_node_key);

        ctx.add_instruction( Instruction::Return );

        ctx.patch_jump_instruction(&jump_instruction_address, &ctx.instructions.len() );
        
    }

    if connected_nodes.len() > 1
    {
        ctx.add_instruction( Instruction::Join );
    }
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
            // @TODO, need to add a warning here to stop compilation, since it might be the case that a port has no defined value yet

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
        }
    }

    pub fn add_instruction(&mut self, instruction: Instruction)
    {
        self.instructions.push(instruction);
    }

    pub fn add_instruction_placeholder(&mut self, instruction: Instruction) -> InstructionAddress
    {
        let instruction_address = self.instructions.len();
        self.instructions.push(instruction);

        instruction_address
    }

    pub fn patch_jump_instruction(&mut self, patch_instruction_address: &InstructionAddress, new_jump_address: &InstructionAddress)
    {
        match &mut self.instructions[*patch_instruction_address]
        {
            Instruction::Jump( instruction_address ) => { *instruction_address = *new_jump_address },
            Instruction::JumpIfFalse( instruction_address, _) => { *instruction_address = *new_jump_address },
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
