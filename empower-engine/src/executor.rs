use std::collections::HashMap;

use crate::{assets::AssetId, compiler::{Instruction, Program}, value::Value};


pub struct Executor
{
    program: Program,

    frame_stack: Vec<GraphFrame>,
}

impl Executor
{
    pub fn new(program: Program) -> Self
    {
        let initial_frame = GraphFrame::new(program.entry_graph_id);
        
        Self
        {
            program,

            frame_stack: Vec::from( vec![initial_frame] ),
        }
    }

    pub fn is_running(&self) -> bool
    {
        !self.frame_stack.is_empty()
    }

    pub fn run(&mut self)
    {
        if self.frame_stack.is_empty()
        {
            return;
        }

        let should_pop_frame =
        {
            let frame = self.frame_stack.last_mut().unwrap();
            let graph = self.program.compiled_graphs.get(&frame.graph_id).unwrap();

            let next_instruction = &graph.instructions.get(frame.next_address).unwrap();

            let mut pop_frame = false;

            match next_instruction
            {
                Instruction::SetConst( register_address, value) =>
                {
                    frame.value_registers.insert(*register_address, value.clone());
                },
                Instruction::Jump(_) => todo!(),
                Instruction::Print( register_address ) =>
                {
                    println!("{}", frame.value_registers.get(register_address).unwrap().to_string());
                },
                Instruction::Return => {
                    pop_frame = true;
                },
            }
    
            frame.next_address += 1;

            pop_frame 
        };

        if should_pop_frame
        {
            self.frame_stack.pop();
        }
    }
}

pub struct GraphFrame
{
    graph_id: AssetId,
    next_address: usize,
    value_registers: HashMap<i32, Value>,
}

impl GraphFrame
{
    pub fn new(graph_id: AssetId) -> Self
    {
        GraphFrame
        {
            graph_id,
            next_address: 0,
            value_registers: HashMap::new(),
        }
    }
}


