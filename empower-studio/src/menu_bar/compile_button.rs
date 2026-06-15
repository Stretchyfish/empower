use empower_engine::compiler::{Program, RegisterAddress};

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

        ui.horizontal(|ui|
        {
            show_value_registers(ui, program);
            show_instructions(ui, program);
        });
    });
}

fn show_value_registers(ui: &mut egui::Ui, program: &Program)
{
    ui.vertical(|ui|
    {
        ui.push_id("value_visualization", |ui|
        {
            ui.heading("Values");
            egui::ScrollArea::vertical()
            .max_height(300.0)
            .show(ui, |ui|
            {
                egui::Grid::new("value_registers_visualization_grid")
                .num_columns(2)
                .spacing([12.0, 4.0])
                .striped(true)
                .show(ui, |ui|
                {
                    ui.label("Addresses");
                    ui.label("Values");
                    ui.end_row();
        
                    for ( register_address, value ) in &program.registers
                    {
                        ui.label(register_address.to_string());
                        ui.label(value.to_string());
                        ui.end_row();
                    }
                });
            });
        });
    });
}

fn show_instructions(ui: &mut egui::Ui, program: &Program)
{
    ui.vertical(|ui|
    {
        ui.push_id("instruction_visualization", |ui|
        {
            ui.heading("Instructions");
            egui::ScrollArea::vertical()
            .max_height(300.0)
            .show(ui, |ui|
            {
                egui::Grid::new("instructions_visualization_grid")
                .num_columns(2)
                .spacing([12.0, 4.0])
                .striped(true)
                .show(ui, |ui|
                {
                    ui.label("Addresses");
                    ui.label("Instructions");
                    ui.end_row();
            
                    for ( instruction_address, instruction ) in program.instructions.iter().enumerate()
                    {
                        ui.label(instruction_address.to_string());
                        ui.label(instruction.to_string());
                        ui.end_row();
                    }
                });
            });
        });
    });
}
