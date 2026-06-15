use std::collections::VecDeque;
use std::collections::HashMap;

use crate::node_graph::NodeGraphKey;
use crate::node_graph::Port;
use crate::node_graph::port;
use crate::project::Project;
use crate::value::Value;

mod debugger;
use debugger::DebugSettings;

mod instructions;
pub use instructions::Instruction;
pub use instructions::InstructionSet;

pub fn debug_compile(project: &Project, debug_settings: &DebugSettings)
{
    
}

pub fn release_compile(project: &Project) -> Result<Program, &'static str>
{
    let mut ctx = CompilerContext::new();
    
    let entry_node_graph = project.assets.get_node_graph(&project.entry_graph);

    if entry_node_graph.is_none()
    {
        return Err( "No entry graph could be loaded" );
    }

    let entry_node_graph = entry_node_graph.unwrap();

    let inverted_connections = invert_connections(&entry_node_graph.connections);

    let mut next_nodes_to_compile = VecDeque::from(entry_node_graph.input_nodes.clone());

    while !next_nodes_to_compile.is_empty()
    {
        let node_to_compile = next_nodes_to_compile.pop_front().unwrap();

        let (node, inputs, outputs) = entry_node_graph.get_node_input_output(node_to_compile).expect("unable to compile node as something is wrong in node");  // @TODO, take a second look at this function

        let input_value_register_addresses = ctx.preallocate_values(&inputs);
        let output_value_register_addresses = ctx.preallocate_values(&outputs);

        let instructions = node.kind.compile(&mut ctx, input_value_register_addresses, output_value_register_addresses);
        ctx.add_instructions(instructions);

        let connected_nodes_to_compile = get_connected_exec_nodes(outputs, &inverted_connections);

        next_nodes_to_compile.extend(connected_nodes_to_compile);
    }

    Ok( ctx.program )
}

pub type RegisterAddress = i32;

pub struct Program
{
    pub registers: HashMap<RegisterAddress, Value>,
    pub instructions: Vec<Instruction>,
}

impl Program
{
    pub fn new() -> Self
    {
        Self
        {
            registers: HashMap::new(),
            instructions: Vec::new(),
        }
    }
}

pub struct CompilerContext
{
    pub program: Program, // @TODO, find a way to have this not be public
    pub cached_ports: HashMap<NodeGraphKey, RegisterAddress>,
}

impl CompilerContext
{
    pub fn new() -> Self
    {
        Self
        {
            program: Program::new(),
            cached_ports: HashMap::new(),
        }
    }

    pub fn add_instructions(&mut self, instructions: Vec<Instruction>)
    {
        self.program.instructions.extend(instructions);
    }

    pub fn preallocate_values(&mut self, ports: &Vec<&Port>) -> Vec<RegisterAddress>
    {
        self.program.registers.reserve(ports.len());

        let mut register_addresses = Vec::new();
        register_addresses.reserve(ports.len());

        for port in ports
        {
            if self.cached_ports.contains_key(&port.key)
            {
                continue;
            }
            
            match port.kind
            {
                port::PortKind::Execution => { continue; },
                port::PortKind::Data => {},
            }

            if port.value.is_none()
            {
                return panic!("Failed to fetch value from port");
            }

            let value = port.value.as_ref().unwrap();

            let new_address = self.get_available_register_address();

            self.program.registers.insert(new_address, value.clone()); 

            register_addresses.push(new_address);
            self.cached_ports.insert(port.key, new_address);
        }

        register_addresses
    }

    fn get_available_register_address(&self) -> RegisterAddress
    {
        self.program.registers.len() as i32
    }

}

fn compile()
{
    
}

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
