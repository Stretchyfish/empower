use std::collections::HashMap;

use crate::{assets::AssetId, compiler::{Instruction, Program}, value::Value};

mod executor_settings;
pub use executor_settings::ExecutorSettings;

pub struct Executor
{
    program: Program,
    pub settings: ExecutorSettings,

    frames: Vec<GraphFrame>,
}

impl Executor
{
    pub fn new(program: Program, settings: ExecutorSettings) -> Self
    {
        let initial_frame = GraphFrame::new(program.entry_graph_id);
        
        Self
        {
            program,

            settings,

            frames: Vec::from( vec![initial_frame] ),
        }
    }

    pub fn is_running(&self) -> bool
    {
        !self.frames.is_empty()
    }

    pub fn run(&mut self)
    {
        if self.frames.is_empty()
        {
            return;
        }

        let mut frames_to_remove = Vec::new();

        for (frame_index, frame) in self.frames.iter_mut().enumerate()
        {
            let graph = self.program.compiled_graphs.get(&frame.graph_id).unwrap();

            let time_now = std::time::Instant::now();

            match frame.state
            {
                GraphFrameState::Running => {},
                GraphFrameState::Sleeping(time_wake) =>
                {
                    if time_now < time_wake
                    {
                        continue;
                    }

                    frame.state = GraphFrameState::Running;
                },
            }
            
            let next_instruction = &graph.instructions.get(frame.next_address).unwrap();

            match next_instruction
            {
                Instruction::SetConst( register_address, value) =>
                {
                    frame.value_registers.insert(*register_address, value.clone());
                },
                Instruction::Copy( from_register_address, to_register_address) =>
                {
                    frame.value_registers.insert( *to_register_address, frame.value_registers.get(from_register_address).unwrap().clone() );
                }
                Instruction::Jump( instruction_address ) =>
                {
                    frame.next_address = *instruction_address - 1;
                },
                Instruction::Print( register_address ) =>
                {
                    let text = frame.value_registers.get(register_address).unwrap().to_string();
                    println!("{}", text);

                    if let Some(outputs) = &mut self.settings.outputs
                    {
                        outputs.push(text);
                    }
                },
                Instruction::Return =>
                {
                    frames_to_remove.push( frame_index );
                },
                Instruction::JumpIfFalse( instruction_address, register_address ) =>
                {
                    if !frame.value_registers.get(register_address).unwrap().as_bool()
                    {
                        frame.next_address = *instruction_address - 1;
                    }
                },
                Instruction::Wait( register_address ) =>
                {
                    frame.state = GraphFrameState::Sleeping( std::time::Instant::now() + std::time::Duration::from_secs_f32( frame.value_registers.get(register_address).unwrap().as_f32() ));
                },
            }
    
            frame.next_address += 1;
        }

        for frame_index in frames_to_remove
        {
            self.frames.remove(frame_index);
        }
    }
}

pub struct GraphFrame
{
    graph_id: AssetId,
    next_address: usize,
    value_registers: HashMap<i32, Value>,
    state: GraphFrameState,
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
            state: GraphFrameState::Running,
        }
    }
}

enum GraphFrameState
{
    Running,
    Sleeping( std::time::Instant ),
}


