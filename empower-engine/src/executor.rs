use std::collections::{HashMap, HashSet};

use crate::{assets::AssetId, compiler::{Instruction, Program}, value::Value};

mod executor_settings;
pub use executor_settings::ExecutorSettings;

mod window_manager;
use window_manager::WindowManager;
use window_manager::WindowType;

pub struct Executor
{
    program: Program,
    pub settings: ExecutorSettings,
    window_manager: WindowManager,

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
            window_manager: WindowManager::new(),

            frames: Vec::from( vec![initial_frame] ),
        }
    }

    pub fn is_running(&self) -> bool
    {
        !self.frames.is_empty()
    }

    pub fn run(&mut self, ui: Option<&mut egui::Ui>)
    {
        if self.frames.is_empty()
        {
            return;
        }

        let mut frames_to_add = Vec::new();
        let mut frames_to_remove = Vec::new();

        for (frame_index, frame) in self.frames.iter_mut().enumerate()
        {
            let graph = self.program.compiled_graphs.get(&frame.graph_id).unwrap();

            let time_now = std::time::Instant::now();

            match &frame.state
            {
                GraphFrameState::Running => {},
                GraphFrameState::Sleeping(time_wake) =>
                {
                    if time_now < *time_wake
                    {
                        continue;
                    }

                    frame.state = GraphFrameState::Running;
                },
                GraphFrameState::ShowImage( window_name ) =>
                {
                    let mut window_got_closed = false;
                    ui.as_ref().unwrap().show_viewport_immediate(
                        egui::ViewportId::from_hash_of(window_name.clone()),
                        egui::ViewportBuilder::default()
                        .with_title(window_name)
                        .with_inner_size([600.0, 400.0]),
                        |ui, _class| {

                            window_got_closed = ui.input(|i| i.viewport().close_requested());
                        },
                    );

                    if !window_got_closed
                    {
                        continue;
                    }

                    self.window_manager.remove_window(window_name);
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
                Instruction::CallGraph( node_graph_id ) =>
                {
                    frames_to_add.push( GraphFrame::new(*node_graph_id) );
                },
                Instruction::ShowImage(_) =>
                {
                    let window_name = self.window_manager.add_new_window("show image".to_string());
                    frame.state = GraphFrameState::ShowImage( window_name );
                },
            }
    
            frame.next_address += 1;
        }

        for frame_index in frames_to_remove
        {
            self.frames.remove(frame_index);
        }

        for frame in frames_to_add
        {
            self.frames.push(frame);
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
    ShowImage( String ),
}


