use std::collections::VecDeque;
use std::collections::HashMap;

use crate::assets::AssetId;
use crate::compiler::instructions::InstructionAddress;
use crate::node_graph::NodeGraph;
use crate::node_graph::NodeGraphKey;
use crate::node_graph::Port;
use crate::node_graph::node::node_kind::ControlFlowKind;
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

pub fn debug_compile(_: &Project, _: &DebugSettings)
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
    let node_graph = project.assets.get_node_graph(&graph_id).unwrap(); 
    let mut compiled_graph_context = CompiledGraphContext::new(graph_id);

    compile_node_chain(&mut compiled_graph_context, node_graph, &node_graph.start_node_key); // This initiates the recursive process to compile the entire graph

    compiled_graph_context.add_instruction( Instruction::Return );

    ctx.add_more_graphs_to_build(&compiled_graph_context.additional_graphs_to_compile);
    ctx.add_compiled_graph(graph_id, CompiledGraph::from_compiled_graph_context(compiled_graph_context));
}

fn compile_node_chain(compiled_graph_context: &mut CompiledGraphContext, node_graph: &NodeGraph, start_node_key: &NodeGraphKey)
{
    let mut next_nodes_to_compile = VecDeque::from( vec![ *start_node_key ]);

    while !next_nodes_to_compile.is_empty()
    {
        let node_to_compile = next_nodes_to_compile.front().unwrap();

        let other_node_to_compile_first = check_if_node_needs_another_node_compiled_first(compiled_graph_context, node_to_compile, &node_graph);

        if other_node_to_compile_first.is_some()
        {
            next_nodes_to_compile.push_front(other_node_to_compile_first.unwrap());
            continue;
        }

        // let connected_exec_ports;

        let (output_port_keys, control_flow) = 
        {
            let (node, inputs, outputs) = node_graph.get_node_input_output(*node_to_compile).expect("unable to compile node as something is wrong in node");  // @TODO, take a second look at this function

            let input_slots = allocate_input_port_registers(compiled_graph_context, &inputs, &node_graph.connections_in);
            let output_slots = allocate_output_port_registers(compiled_graph_context, &outputs);

            node.kind.compile(compiled_graph_context, input_slots, output_slots);

            (node.output_port_keys.clone(), node.kind.control_flow())
        };

        next_nodes_to_compile.pop_front();

        if output_port_keys.is_empty() // This is for nodes with no output ports
        {
            continue;
        }

        match control_flow
        {
            ControlFlowKind::Normal =>
            {
                next_nodes_to_compile.extend( get_nodes_connected_to_port(output_port_keys[0], node_graph) );
            },
            ControlFlowKind::Branch =>
            {
                let nodes_connected_to_true_branch = get_nodes_connected_to_port(output_port_keys[0], node_graph);
                let nodes_connected_to_false_branch = get_nodes_connected_to_port(output_port_keys[1], node_graph);

                if nodes_connected_to_true_branch.is_empty() && nodes_connected_to_false_branch.is_empty()
                {
                    compiled_graph_context.instructions.pop();
                    continue;
                }

                let jump_if_false_instruction_placeholder_address = compiled_graph_context.get_latest_instruction_address();

                for node_key in nodes_connected_to_true_branch
                {
                    compile_node_chain(compiled_graph_context, node_graph, &node_key);
                }

                let jump_after_true_branch_placeholder_address = compiled_graph_context.add_instruction_placeholder( Instruction::Jump(0) );
                compiled_graph_context.patch_jump_instruction(&jump_if_false_instruction_placeholder_address, &(jump_after_true_branch_placeholder_address + 1));

                for node_key in nodes_connected_to_false_branch
                {
                    compile_node_chain(compiled_graph_context, node_graph, &node_key);
                }

                let instruction_after_false_branch_address = compiled_graph_context.instructions.len(); // This makes the dangerous assumption that there always is an instruction after. That should be the case with the current implementation.

                compiled_graph_context.patch_jump_instruction(&jump_after_true_branch_placeholder_address, &instruction_after_false_branch_address);
            },
            ControlFlowKind::Loop =>
            {
                let nodes_connected_to_exec_port = get_nodes_connected_to_port(output_port_keys[0], node_graph);

                if nodes_connected_to_exec_port.is_empty()
                {
                    continue;
                }

                let first_instruction_in_loop_address = compiled_graph_context.get_latest_instruction_address() + 1; // this might have issues if there are no next instructions (stay aware of this in the future)

                for node_key in nodes_connected_to_exec_port
                {
                    compile_node_chain(compiled_graph_context, node_graph, &node_key);
                }

                compiled_graph_context.add_instruction( Instruction::Jump(first_instruction_in_loop_address) );
            },
        }
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
