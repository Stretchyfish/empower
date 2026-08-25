use std::collections::HashMap;

use crate::assets::AssetId;
use crate::assets::LoadedAssets;
use crate::node_graph::NodeGraph;
use crate::node_graph::NodeGraphKey;
use crate::node_graph::Port;
use crate::node_graph::node::NodeKind;
use crate::node_graph::node::node_kind::LoopMode;
use crate::node_graph::port;
use crate::node_graph::port::PortKind;
use crate::project::Project;
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

    let integrity = check_node_graph_compile_integrity(node_graph);

    if integrity.is_err()
    {
        return integrity;
    }
    
    ctx.add_assets_to_load( detect_assets_to_preload(node_graph) );

    let mut compiled_graph_context = CompiledGraphContext::new(graph_id);

    compile_node_chain(&mut compiled_graph_context, node_graph, &node_graph.start_node_key); // This initiates the recursive process to compile the entire graph

    compiled_graph_context.add_instruction( &node_graph.start_node_key, Instruction::Return ); 

    ctx.add_more_graphs_to_build(&compiled_graph_context.additional_graphs_to_compile);

    let ( compiled_graph, trace ) = ( CompiledGraph::from(compiled_graph_context.register_size, compiled_graph_context.instructions), compiled_graph_context.trace );

    ctx.add_traced_data( graph_id, trace ); // @TODO, improve this
    ctx.add_compiled_graph(graph_id, compiled_graph);

    Ok(())
}

fn compile_node_chain(ctx: &mut CompiledGraphContext, node_graph: &NodeGraph, node_key: &NodeGraphKey)
{
    let next_instruction_address = ctx.instructions.len(); // @TODO, maybe find a better name, gets confusing in loop
    
    let other_node_to_compile_first = check_if_node_needs_another_node_compiled_first(ctx, node_key, &node_graph);

    if other_node_to_compile_first.is_some()
    {
        compile_node_chain(ctx, node_graph, &other_node_to_compile_first.unwrap());
    }

    let (node, inputs, outputs) = node_graph.get_node_input_output(*node_key).expect("unable to compile node as something is wrong in node");  // @TODO, take a second look at this function

    let input_register_addresses = allocate_input_port_registers(ctx, node_key, &inputs, &node_graph.connections_in);
    let output_register_addresses = allocate_output_port_registers(ctx, &outputs);

    match &node.kind
    {
        NodeKind::Start =>
        {
            compile_nodes_connected_to_port(ctx, node_graph, &node.output_port_keys[0], node_key);
        },
        NodeKind::Print =>
        {
            ctx.add_instruction( node_key, Instruction::Print( input_register_addresses[0] ) );
        },
        NodeKind::Branch =>
        {
            let jump_if_false_instruction_placeholder_address = ctx.add_instruction_placeholder( node_key, Instruction::JumpIfFalse(0, input_register_addresses[0]));
            
            compile_nodes_connected_to_port(ctx, node_graph, &node.output_port_keys[0], node_key);

            let jump_after_true_branch_instruction_placeholder_address = ctx.add_instruction_placeholder( node_key,  Instruction::Jump(0) );
            ctx.patch_jump_instruction(&jump_if_false_instruction_placeholder_address, &ctx.instructions.len());

            compile_nodes_connected_to_port(ctx, node_graph, &node.output_port_keys[1], node_key);

            ctx.patch_jump_instruction(&jump_after_true_branch_instruction_placeholder_address , &ctx.instructions.len());
        },
        NodeKind::Loop( mode ) =>
        {
            match mode
            {
                LoopMode::Forever =>
                {
                    compile_nodes_connected_to_port(ctx, node_graph, &node.output_port_keys[0], node_key);

                    ctx.add_instruction( node_key, Instruction::Jump( next_instruction_address ) );
                },
                LoopMode::Range =>
                {
                    ctx.add_instruction( node_key, Instruction::Copy(input_register_addresses [0], output_register_addresses [1]) );
                    ctx.add_instruction( node_key, Instruction::Compare(input_register_addresses [2], output_register_addresses [1], output_register_addresses [0] ) );
                    let jump_if_true_placeholder_address = ctx.add_instruction_placeholder( node_key, Instruction::JumpIfTrue(0, output_register_addresses [0]));

                    compile_nodes_connected_to_port(ctx, node_graph, &node.output_port_keys[0], node_key);

                    ctx.add_instruction( node_key, Instruction::Add(output_register_addresses [1], input_register_addresses [1], output_register_addresses [1]));
                    ctx.add_instruction( node_key, Instruction::Jump( jump_if_true_placeholder_address - 1 ) );

                    ctx.patch_jump_instruction(&jump_if_true_placeholder_address , &ctx.get_next_instruction_address());
                },
            }
        },
        NodeKind::Wait =>
        {
            ctx.add_instruction( node_key, Instruction::Wait( input_register_addresses[0] ) );

            compile_nodes_connected_to_port(ctx, node_graph, &node.output_port_keys[0], node_key);
        },
        NodeKind::List(_) =>
        {
            ctx.add_instruction( node_key, Instruction::CreateList( input_register_addresses.clone(), output_register_addresses[0]) );
        },
        NodeKind::Image( state ) =>
        {
            ctx.add_instruction( node_key, Instruction::SetConst(output_register_addresses[0], Value::Image( state.image_asset_id ))
            );
        },
        NodeKind::ShowImage =>
        {
            ctx.add_instruction( node_key, Instruction::ShowImage( input_register_addresses[0] ) );
        },
        NodeKind::MathGraph =>
        {
            ctx.add_instruction( node_key, Instruction::ShowMathGraph( input_register_addresses[0] ) );
        },
        NodeKind::SubGraph( state ) =>
        {
            if state.graph_asset_id.is_none() // @TODO, double check that this still works
            {
                return;
            }

            let graph_id = state.graph_asset_id.unwrap();

            ctx.additional_graphs_to_compile.push(graph_id);
            ctx.add_instruction( node_key, Instruction::CallGraph(graph_id));
        },
    }
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

fn compile_nodes_connected_to_port(ctx: &mut CompiledGraphContext, node_graph: &NodeGraph, port_key: &NodeGraphKey, node_key: &NodeGraphKey)
{
    let connected_nodes = get_nodes_connected_to_port(*port_key, node_graph);

    for (index, connected_node_key) in connected_nodes.iter().enumerate()
    {
        if index == connected_nodes.len() - 1
        {
            compile_node_chain(ctx, node_graph, connected_node_key);
            continue;
        }

        ctx.add_instruction( node_key, Instruction::Fork( ctx.instructions.len() + 2 ));
        let jump_instruction_address = ctx.add_instruction_placeholder( node_key, Instruction::Jump( 0) );

        compile_node_chain(ctx, node_graph, connected_node_key);

        ctx.add_instruction( node_key, Instruction::Return );

        ctx.patch_jump_instruction(&jump_instruction_address, &ctx.instructions.len() );
    }

    if connected_nodes.len() > 1
    {
        ctx.add_instruction( node_key, Instruction::Join );
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

