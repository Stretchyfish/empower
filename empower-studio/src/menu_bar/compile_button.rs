use empower_engine::{compiler::Instruction, node_graph::NodeAddress};

use crate::studio_context::StudioContext;

const REGISTER_ADDRESS_TEXT_COLOR: egui::Color32 = egui::Color32::BLUE;
const INSTRUCTION_ADDRESS_TEXT_COLOR: egui::Color32 = egui::Color32::YELLOW;
const ASSET_ID_TEXT_COLOR: egui::Color32 = egui::Color32::GREEN;
const VALUE_TEXT_COLOR: egui::Color32 = egui::Color32::MAGENTA;

pub fn show(ui: &mut egui::Ui, studio_context: &mut StudioContext)
{
    if ui.button("🔨 Compile").clicked()
    {
        studio_context.request_compile();
    }

    let compile_result = studio_context.get_compile_result();

    if compile_result.is_none()
    {
        return;
    }

    let mut hovered_graph_and_node = None;

    let compile_menu_response = ui.menu_button("⚪", |ui|
    {
        let program = &compile_result.as_ref().unwrap().program;
        let meta = &compile_result.as_ref().unwrap().meta;

        ui.heading("Compiled graphs");

        ui.label(format!("entry graph: {}", program.entry_graph_id));

        egui::ScrollArea::vertical()
        .max_height(300.0)
        .show(ui, |ui|
        {
            for (graph_id, compiled_graph) in &program.compiled_graphs
            {
                ui.menu_button(format!("graph: {}", graph_id), |ui|
                {
                    egui::ScrollArea::vertical()
                    .max_height(300.0)
                    .show(ui, |ui|
                    {
                        ui.label(format!("registers: {}", compiled_graph.register_size));

                        ui.horizontal(|ui|
                        {
                            let (rect, _) = ui.allocate_exact_size( egui::vec2(5.0, 5.0), egui::Sense::hover() );
                            ui.painter().circle(rect.center(), 5.0, INSTRUCTION_ADDRESS_TEXT_COLOR, egui::Stroke::NONE);
                            ui.label(" = instruction address");
                        });

                        ui.horizontal(|ui|
                        {
                            let (rect, _) = ui.allocate_exact_size( egui::vec2(5.0, 5.0), egui::Sense::hover() );
                            ui.painter().circle(rect.center(), 5.0, REGISTER_ADDRESS_TEXT_COLOR, egui::Stroke::NONE);
                            ui.label(" = register address");
                        });

                        ui.horizontal(|ui|
                        {
                            let (rect, _) = ui.allocate_exact_size( egui::vec2(5.0, 5.0), egui::Sense::hover() );
                            ui.painter().circle(rect.center(), 5.0, ASSET_ID_TEXT_COLOR, egui::Stroke::NONE);
                            ui.label(" = asset id");
                        });

                        ui.horizontal(|ui|
                        {
                            let (rect, _) = ui.allocate_exact_size( egui::vec2(5.0, 5.0), egui::Sense::hover() );
                            ui.painter().circle(rect.center(), 5.0, VALUE_TEXT_COLOR, egui::Stroke::NONE);
                            ui.label(" = value");
                        });

                        egui::Grid::new("graph_instructions_visualization")
                        .num_columns(2)
                        .spacing([12.0, 4.0])
                        .striped(true)
                        .show(ui, |ui|
                        {
                            for ( instruction_address, instruction) in compiled_graph.instructions.iter().enumerate()
                            {
                                ui.label(instruction_address.to_string());
                                let instruction_layout_response =  ui.label(instruction_as_layout_job(instruction));

                                if instruction_layout_response.hovered()
                                {
                                    let instruction_node_address = meta.as_ref().unwrap().trace.get(graph_id).unwrap().get(&instruction_address).expect("trace doesn't contain this node");
                                    // potential_new_user_state = Some( UserState::HighlightingNode { graph_id: 1, node_key: *instruction_node } );
                                    hovered_graph_and_node = Some( NodeAddress { graph_id: *graph_id, node_key: *instruction_node_address } );
                                    // studio_context.get_cache_mut().highlighted_nodes = vec![ *instruction_node ];
                                }

                                ui.end_row();
                            }
                        });
                    });
                });
            }
        });
    });

    {
        let cache = studio_context.get_cache_mut();

        if hovered_graph_and_node.is_some()
        {
            cache.session.instruction_highlighted_nodes = hovered_graph_and_node;
        }

        if compile_menu_response.inner.is_none()
        {
            studio_context.get_cache_mut().session.instruction_highlighted_nodes = None;
        }
    }
}

fn instruction_as_layout_job(instruction: &Instruction) -> egui::text::LayoutJob
{
    let mut job = egui::text::LayoutJob::default();

    match instruction
    {
        Instruction::SetConst( register_address, value) =>
        {
            job.append("SetConst( ", 0.0, egui::TextFormat::default() );
            job.append(register_address.to_string().as_str(), 0.0, egui::TextFormat
            {
                color: REGISTER_ADDRESS_TEXT_COLOR,
                ..Default::default()
            });
            job.append(", ", 0.0, egui::TextFormat::default() );
            job.append(value.to_string().as_str(), 0.0, egui::TextFormat
            {
                color: VALUE_TEXT_COLOR,
                ..Default::default()
            });
            job.append(" )", 0.0, egui::TextFormat::default() );
        },
        Instruction::Copy( register_address1, register_address2 ) =>
        {
            job.append("Copy( ", 0.0, egui::TextFormat::default() );
            job.append(register_address1.to_string().as_str(), 0.0, egui::TextFormat
            {
                color: REGISTER_ADDRESS_TEXT_COLOR,
                ..Default::default()
            });
            job.append(", ", 0.0, egui::TextFormat::default() );
            job.append(register_address2.to_string().as_str(), 0.0, egui::TextFormat
            {
                color: REGISTER_ADDRESS_TEXT_COLOR,
                ..Default::default()
            });
            job.append(" )", 0.0, egui::TextFormat::default() );
        },
        Instruction::Jump( instruction_address ) =>
        {
            job.append("Jump( ", 0.0, egui::TextFormat::default() );
            job.append(instruction_address.to_string().as_str(), 0.0, egui::TextFormat
            {
                color: INSTRUCTION_ADDRESS_TEXT_COLOR,
                ..Default::default()
            });
            job.append(" )", 0.0, egui::TextFormat::default() );
        },
        Instruction::JumpIfFalse( instruction_address, register_address) =>
        {
            job.append("JumpIfFalse( ", 0.0, egui::TextFormat::default() );
            job.append(instruction_address.to_string().as_str(), 0.0, egui::TextFormat
            {
                color: INSTRUCTION_ADDRESS_TEXT_COLOR,
                ..Default::default()
            });
            job.append(", ", 0.0, egui::TextFormat::default() );
            job.append(register_address.to_string().as_str(), 0.0, egui::TextFormat
            {
                color: REGISTER_ADDRESS_TEXT_COLOR,
                ..Default::default()
            });
            job.append(" )", 0.0, egui::TextFormat::default() );
        },
        Instruction::Print( register_address ) =>
        {
            job.append("Print( ", 0.0, egui::TextFormat::default() );
            job.append(register_address.to_string().as_str(), 0.0, egui::TextFormat
            {
                color: REGISTER_ADDRESS_TEXT_COLOR,
                ..Default::default()
            });
            job.append(" )", 0.0, egui::TextFormat::default() );
        },
        Instruction::Return =>
        {
            job.append("Return", 0.0, egui::TextFormat::default() );
        },
        Instruction::Wait( register_address ) =>
        {
            job.append("Wait( ", 0.0, egui::TextFormat::default() );
            job.append(register_address.to_string().as_str(), 0.0, egui::TextFormat
            {
                color: REGISTER_ADDRESS_TEXT_COLOR,
                ..Default::default()
            });
            job.append(" )", 0.0, egui::TextFormat::default() );
        },
        Instruction::CallGraph( asset_id ) =>
        {
            job.append("CallGraph( ", 0.0, egui::TextFormat::default() );
            job.append(asset_id.to_string().as_str(), 0.0, egui::TextFormat
            {
                color: ASSET_ID_TEXT_COLOR,
                ..Default::default()
            });
            job.append(" )", 0.0, egui::TextFormat::default() );
        },
        Instruction::ShowImage( register_address ) =>
        {
            job.append("ShowImage( ", 0.0, egui::TextFormat::default() );
            job.append(register_address.to_string().as_str(), 0.0, egui::TextFormat
            {
                color: REGISTER_ADDRESS_TEXT_COLOR,
                ..Default::default()
            });
            job.append(" )", 0.0, egui::TextFormat::default() );
        },
        Instruction::Fork( instruction_address ) =>
        {
            job.append("Fork( ", 0.0, egui::TextFormat::default() );
            job.append(instruction_address.to_string().as_str(), 0.0, egui::TextFormat
            {
                color: INSTRUCTION_ADDRESS_TEXT_COLOR,
                ..Default::default()
            });
            job.append(" )", 0.0, egui::TextFormat::default() );
        },
        Instruction::Join =>
        {
            job.append("Join", 0.0, egui::TextFormat::default() );
        },
        Instruction::ShowMathGraph( register_address ) =>
        {
            job.append("ShowMathGraph( ", 0.0, egui::TextFormat::default() );
            job.append(register_address.to_string().as_str(), 0.0, egui::TextFormat
            {
                color: REGISTER_ADDRESS_TEXT_COLOR,
                ..Default::default()
            });
            job.append(" )", 0.0, egui::TextFormat::default() );
        }
        Instruction::CreateList( list_element_register_addresses, to_register_address ) =>
        {
            job.append("CreateList( ", 0.0, egui::TextFormat::default() );
            job.append(format!("{:?}", list_element_register_addresses).as_str(), 0.0, egui::TextFormat
            {
                color: REGISTER_ADDRESS_TEXT_COLOR,
                ..Default::default()
            });
            job.append(", ", 0.0, egui::TextFormat::default() );
            job.append(to_register_address.to_string().as_str(), 0.0, egui::TextFormat
            {
                color: REGISTER_ADDRESS_TEXT_COLOR,
                ..Default::default()
            });
            job.append(" )", 0.0, egui::TextFormat::default() );
        }
        Instruction::Add(register_address_1, register_address_2, result_register_address ) =>
        {
            job.append("Add( ", 0.0, egui::TextFormat::default() );
            job.append(register_address_1.to_string().as_str(), 0.0, egui::TextFormat
            {
                color: REGISTER_ADDRESS_TEXT_COLOR,
                ..Default::default()
            });
            job.append(", ", 0.0, egui::TextFormat::default() );
            job.append(register_address_2.to_string().as_str(), 0.0, egui::TextFormat
            {
                color: REGISTER_ADDRESS_TEXT_COLOR,
                ..Default::default()
            });
            job.append(", ", 0.0, egui::TextFormat::default() );
            job.append(result_register_address.to_string().as_str(), 0.0, egui::TextFormat
            {
                color: REGISTER_ADDRESS_TEXT_COLOR,
                ..Default::default()
            });
            job.append(" )", 0.0, egui::TextFormat::default() );
        },
        Instruction::Compare(register_address_1, register_address_2, result_register_address) =>
        {
            job.append("Compare( ", 0.0, egui::TextFormat::default() );
            job.append(register_address_1.to_string().as_str(), 0.0, egui::TextFormat
            {
                color: REGISTER_ADDRESS_TEXT_COLOR,
                ..Default::default()
            });
            job.append(", ", 0.0, egui::TextFormat::default() );
            job.append(register_address_2.to_string().as_str(), 0.0, egui::TextFormat
            {
                color: REGISTER_ADDRESS_TEXT_COLOR,
                ..Default::default()
            });
            job.append(", ", 0.0, egui::TextFormat::default() );
            job.append(result_register_address.to_string().as_str(), 0.0, egui::TextFormat
            {
                color: REGISTER_ADDRESS_TEXT_COLOR,
                ..Default::default()
            });
            job.append(" )", 0.0, egui::TextFormat::default() );
        },
        Instruction::JumpIfTrue( instruction_address, register_address) =>
        {
            job.append("JumpIfTrue( ", 0.0, egui::TextFormat::default() );
            job.append(instruction_address.to_string().as_str(), 0.0, egui::TextFormat
            {
                color: INSTRUCTION_ADDRESS_TEXT_COLOR,
                ..Default::default()
            });
            job.append(", ", 0.0, egui::TextFormat::default() );
            job.append(register_address.to_string().as_str(), 0.0, egui::TextFormat
            {
                color: REGISTER_ADDRESS_TEXT_COLOR,
                ..Default::default()
            });
            job.append(" )", 0.0, egui::TextFormat::default() );
        },
    }

    job
}
