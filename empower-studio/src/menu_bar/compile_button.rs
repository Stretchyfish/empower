use crate::studio_context::StudioContext;

pub fn show(ui: &mut egui::Ui, studio_context: &mut StudioContext)
{
    if ui.button("🔨 Compile").clicked()
    {
        studio_context.request_compile();
    }

    let program = studio_context.get_program();

    if program.is_none()
    {
        return;
    }

    ui.menu_button("⚪", |ui|
    {
        let program = program.as_ref().unwrap();

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

                        egui::Grid::new("graph_instructions_visualization")
                        .num_columns(2)
                        .spacing([12.0, 4.0])
                        .striped(true)
                        .show(ui, |ui|
                        {
                            for ( instruction_address, instruction) in compiled_graph.instructions.iter().enumerate()
                            {
                                ui.label(instruction_address.to_string());
                                ui.label(instruction.to_string());
                                ui.end_row();
                            }
                        });
                    });
                });
            }
        });
    });
}
