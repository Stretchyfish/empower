use std::collections::HashMap;

use crate::{assets::AssetId, compiler::{Instruction, Program}, value::Value};

mod executor_settings;
pub use executor_settings::ExecutorSettings;

mod window_manager;
use window_manager::WindowManager;

type FrameIndex = usize;
type PointerIndex = usize;

type InstructionAddress = usize;

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

        let mut execution_actions = Vec::new();

        for (frame_index, frame) in self.frames.iter_mut().enumerate()
        {
            let graph = self.program.compiled_graphs.get(&frame.graph_id).unwrap();

            let time_now = std::time::Instant::now();

            for (pointer_index, pointer) in frame.pointers.iter_mut().enumerate()
            {
                match &pointer.state
                {
                    InstructionPointerState::Running => {},
                    InstructionPointerState::Sleeping(time_wake) =>
                    {
                        if time_now < *time_wake
                        {
                            continue;
                        }

                        pointer.state = InstructionPointerState::Running;
                    },
                    InstructionPointerState::ShowImage( window_name ) =>
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

                let next_instruction = &graph.instructions.get(pointer.next_address).unwrap();

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
                        pointer.next_address = *instruction_address - 1;
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
                        execution_actions.push( ExecutionAction::RemovePointer( frame_index, pointer_index ) );
                    },
                    Instruction::JumpIfFalse( instruction_address, register_address ) =>
                    {
                        if !frame.value_registers.get(register_address).unwrap().as_bool()
                        {
                            pointer.next_address = *instruction_address - 1;
                        }
                    },
                    Instruction::Wait( register_address ) =>
                    {
                        pointer.state = InstructionPointerState::Sleeping( std::time::Instant::now() + std::time::Duration::from_secs_f32( frame.value_registers.get(register_address).unwrap().as_f32() ));
                    },
                    Instruction::CallGraph( node_graph_id ) =>
                    {
                        execution_actions.push( ExecutionAction::AddFrame( *node_graph_id ) );
                    },
                    Instruction::ShowImage(_) =>
                    {
                        let window_name = self.window_manager.add_new_window("show image".to_string());
                        pointer.state = InstructionPointerState::ShowImage( window_name );
                    },
                    Instruction::Fork( instruction_address ) =>
                    {
                        execution_actions.push( ExecutionAction::AddPointer( frame_index, *instruction_address ) );
                    },
                }
    
                pointer.next_address += 1;
            }
        }

        for action in execution_actions
        {
            match action
            {
                ExecutionAction::AddFrame( node_graph_asset_id ) =>
                {
                    self.frames.push( GraphFrame::new(node_graph_asset_id) );
                },
                ExecutionAction::RemoveFrame( frame_index ) =>
                {
                    self.frames.remove( frame_index );
                },
                ExecutionAction::AddPointer( frame_index, start_instruction_address ) =>
                {
                    self.frames[frame_index].pointers.push( InstructionPointer::from( start_instruction_address ) ); // This is potentially slightly dangerous in the future, as it might access a frame that has been removed
                },
                ExecutionAction::RemovePointer( frame_index, pointer_index ) =>
                {
                    self.frames[frame_index].pointers.remove(pointer_index); // Also potentially dangerous

                    if self.frames[frame_index].pointers.is_empty()
                    {
                        self.frames.remove(frame_index);
                    }
                },
            }
        }
    }
}

pub struct GraphFrame
{
    graph_id: AssetId,
    value_registers: HashMap<i32, Value>,
    pointers: Vec<InstructionPointer>,
}

impl GraphFrame
{
    pub fn new(graph_id: AssetId) -> Self
    {
        GraphFrame
        {
            graph_id,
            value_registers: HashMap::new(),
            pointers: vec![ InstructionPointer::new() ],
        }
    }
}

struct InstructionPointer
{
    next_address: usize,
    state: InstructionPointerState,
}

impl InstructionPointer
{
    pub fn new() -> Self
    {
        Self
        {
            next_address: 0,
            state: InstructionPointerState::Running,
        }
    }

    pub fn from(instruction_address: usize) -> Self
    {
        Self
        {
            next_address: instruction_address,
            state: InstructionPointerState::Running,
        }
    }
}

enum InstructionPointerState
{
    Running,
    Sleeping( std::time::Instant ),
    ShowImage( String ),
}

enum ExecutionAction
{
    AddFrame( AssetId ),
    RemoveFrame( FrameIndex ),
    AddPointer( FrameIndex, InstructionAddress ),
    RemovePointer( FrameIndex, PointerIndex ),
}

