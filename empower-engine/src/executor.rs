use egui_plot::{Legend, Line, Plot, PlotPoints};
use std::collections::HashMap;

use crate::{assets::AssetId, compiler::{Instruction, Program}, value::Value};

mod executor_settings;
pub use executor_settings::ExecutorSettings;

mod executor_output;
pub use executor_output::ExecutorOutput;
pub use executor_output::ExecuteUnit;

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

    pub fn run(&mut self, ui: Option<&mut egui::Ui>) -> ExecutorOutput
    {
        let mut execution_output = ExecutorOutput::new(); // @TODO, sice of instructions can be preallocated to the number of pointers
        
        if self.frames.is_empty()
        {
            return execution_output;
        }

        let mut execution_actions = Vec::new();

        for (frame_index, frame) in self.frames.iter_mut().enumerate()
        {
            let graph = self.program.compiled_graphs.get(&frame.graph_id).unwrap();

            let time_now = std::time::Instant::now();

            for (pointer_index, pointer) in frame.pointers.iter_mut().enumerate()
            {
                match &pointer.state // @TODO, consider moving this state bahavior into the instructions
                {
                    InstructionPointerState::Running => {},
                    InstructionPointerState::Sleeping(time_wake) =>
                    {
                        if time_now < *time_wake
                        {
                            execution_output.execute_units.push( ExecuteUnit {
                                                                        graph_id: frame.graph_id,
                                                                        instruction_address: pointer.next_address - 1
                                                                        }
                                                                    ); // @TODo, look into if this can be done in another way
                            continue;
                        }

                        pointer.state = InstructionPointerState::Running;
                    },
                    InstructionPointerState::ShowImage( window_name, image_asset_id ) =>
                    {
                        let mut window_got_closed = false;
                        ui.as_ref().unwrap().show_viewport_immediate(
                            egui::ViewportId::from_hash_of(window_name.clone()),
                            egui::ViewportBuilder::default()
                            .with_title(window_name)
                            .with_inner_size([600.0, 400.0]),
                            |ui, _class| {

                                window_got_closed = ui.input(|i| i.viewport().close_requested());

                                if image_asset_id.is_none()
                                {
                                    
                                }
                                else
                                {
                                    let image = self.program.loaded_assets.loaded_images.get(&image_asset_id.unwrap()).expect("image asset id is not in loaded_assets");

                                    let image_texture = ui.load_texture("image", image.clone(), Default::default());
                                    ui.image(&image_texture);
                                }
                            },
                        );

                        if !window_got_closed
                        {
                            continue;
                        }

                        self.window_manager.remove_window(window_name);
                        pointer.state = InstructionPointerState::Running;
                    },
                    InstructionPointerState::ShowMathGraph( window_name, graph ) =>
                    {
                        let mut window_got_closed = false;
                        ui.as_ref().unwrap().show_viewport_immediate(
                            egui::ViewportId::from_hash_of(window_name.clone()),
                            egui::ViewportBuilder::default()
                            .with_title(window_name)
                            .with_inner_size([600.0, 400.0]),
                            |ui, _class| {

                                window_got_closed = ui.input(|i| i.viewport().close_requested());

                                Plot::new("My Plot")
                                .legend(Legend::default())
                                .show(ui, |plot_ui| 
                                {
                                    plot_ui.line(Line::new(
                                        "Graph",
                                        PlotPoints::from(graph.clone()),
                                    ));
                                });
                            },
                        );

                        if !window_got_closed
                        {
                            continue;
                        }

                        self.window_manager.remove_window(window_name);
                        pointer.state = InstructionPointerState::Running;
                    }
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
                    Instruction::CreateList( list_element_register_addresses, to_register_address ) =>
                    {
                        let mut elements = Vec::with_capacity(list_element_register_addresses.len());

                        for register_address in list_element_register_addresses
                        {
                            elements.push( frame.value_registers.get( register_address).unwrap().clone() );
                        }

                        frame.value_registers.insert( *to_register_address, Value::List( elements ) );
                    }
                    Instruction::Jump( instruction_address ) =>
                    {
                        pointer.next_address = *instruction_address;
                        continue; // To skip the pointer increment in the end
                    },
                    Instruction::Print( register_address ) =>
                    {
                        let text = frame.value_registers.get(register_address).unwrap().to_string();
                        println!("{}", text);

                        execution_output.outputs.push(text);
                    },
                    Instruction::Return =>
                    {
                        execution_actions.push( ExecutionAction::RemovePointer( frame_index, pointer_index ) );
                    },
                    Instruction::JumpIfFalse( instruction_address, register_address ) =>
                    {
                        if !frame.value_registers.get(register_address).unwrap().as_bool()
                        {
                            pointer.next_address = *instruction_address;
                            continue; // To skip the pointer increment in the end
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
                    Instruction::ShowImage( register_address ) =>
                    {
                        let image_asset_id = frame.value_registers.get( register_address ).unwrap().as_asset_id();
                        
                        let window_name = self.window_manager.add_new_window("show image".to_string());
                        pointer.state = InstructionPointerState::ShowImage( window_name, image_asset_id );
                    },
                    Instruction::Fork( instruction_address ) =>
                    {
                        execution_actions.push( ExecutionAction::AddPointer( frame_index, pointer_index, *instruction_address ) );
                    },
                    Instruction::Join =>
                    {
                        if pointer.active_children > 0
                        {
                            continue;
                        }
                    },
                    Instruction::ShowMathGraph( register_address ) =>
                    {
                        let list = frame.value_registers.get( register_address ).unwrap().as_list();
                        
                        let mut x = Vec::with_capacity(list.len());
                        let mut y = Vec::with_capacity(list.len());

                        for point_value in list
                        {
                            let point = match point_value
                            {
                                Value::Point2d( x, y) => (x, y),
                                _ => panic!("tried to read invalid value in show math graph"),
                            };

                            x.push(point.0);
                            y.push(point.1);
                        }
                        
                        let graph = x.iter().zip(y.iter()).map(|(&x1, &y1)| [x1 as f64, y1 as f64]).collect();

                        let window_name = self.window_manager.add_new_window("show math graph".to_string());
                        pointer.state = InstructionPointerState::ShowMathGraph( window_name, graph );
                    }
                    Instruction::Add( register_address_1, register_address_2, result_register_address ) =>
                    {
                        // @TODO, make this add work on multiple value types
                        let added_result = frame.value_registers[register_address_1].as_i32() + frame.value_registers[register_address_2].as_i32();
                        frame.value_registers.insert(*result_register_address, Value::Integer( added_result ));
                        
                    },
                    Instruction::Compare( register_address_1, register_address_2, result_register_address ) =>
                    {
                        let compare_result = frame.value_registers[register_address_1] == frame.value_registers[register_address_2];
                        frame.value_registers.insert(*result_register_address, Value::Bool(compare_result));
                    },
                    Instruction::JumpIfTrue( instruction_address, register_address ) =>
                    {
                        if frame.value_registers.get(register_address).unwrap().as_bool()
                        {
                            pointer.next_address = *instruction_address;
                            continue; // To skip the pointer increment in the end
                        }
                    },
                }

                if self.settings.artificial_delay.is_some()
                {
                    match pointer.state
                    {
                        InstructionPointerState::Sleeping(instant) =>
                        {
                            pointer.state = InstructionPointerState::Sleeping( instant + self.settings.artificial_delay.unwrap() );
                        },
                        _ => {
                            pointer.state = InstructionPointerState::Sleeping( std::time::Instant::now() + self.settings.artificial_delay.unwrap() );
                        }
                    }
                }

                execution_output.execute_units.push( ExecuteUnit { graph_id: frame.graph_id, instruction_address: pointer.next_address } );
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
                // ExecutionAction::RemoveFrame( frame_index ) =>
                // {
                //     self.frames.remove( frame_index );
                // },
                ExecutionAction::AddPointer( frame_index, parent_pointer_index, start_instruction_address ) =>
                {
                    self.frames[frame_index].pointers[parent_pointer_index].active_children += 1;
                    self.frames[frame_index].pointers.push( InstructionPointer::from( start_instruction_address, parent_pointer_index ) ); // This is potentially slightly dangerous in the future, as it might access a frame that has been removed
                },
                ExecutionAction::RemovePointer( frame_index, pointer_index ) =>
                {
                    let frame = self.frames.get_mut(frame_index).unwrap();

                    let removed_pointer = frame.pointers.remove(pointer_index);

                    if removed_pointer.parent.is_some()
                    {
                        frame.pointers.get_mut(removed_pointer.parent.unwrap()).unwrap().active_children -= 1;
                    }

                    if self.frames[frame_index].pointers.is_empty()
                    {
                        self.frames.remove(frame_index);
                    }
                },
            }
        }

        execution_output
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

    parent: Option<usize>,
    active_children: usize,
}

impl InstructionPointer
{
    pub fn new() -> Self
    {
        Self
        {
            next_address: 0,
            state: InstructionPointerState::Running,

            parent: None,
            active_children: 0,
        }
    }

    pub fn from(instruction_address: usize, parent: usize) -> Self
    {
        Self
        {
            next_address: instruction_address,
            state: InstructionPointerState::Running,

            parent: Some( parent),
            active_children: 0,
        }
    }
}

enum InstructionPointerState
{
    Running,
    Sleeping( std::time::Instant ),
    ShowImage( String, Option<AssetId> ),
    ShowMathGraph ( String, Vec<[f64; 2]>),
}

enum ExecutionAction
{
    AddFrame( AssetId ),
    // RemoveFrame( FrameIndex ), // @TODO, add this back with a "stop" node
    AddPointer( FrameIndex, PointerIndex, InstructionAddress ),
    RemovePointer( FrameIndex, PointerIndex ),
}

