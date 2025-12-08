use better_empower_engine::{EmpowerRuntime, runtime::EmpowerExecutor};

use crate::StudioContext;

pub fn show(ui: &mut egui::Ui, studio_context: &mut StudioContext)
{
    egui::MenuBar::new()
    .ui(ui, |ui|
    {
        ui.menu_button("Nodes", |ui|
        {
            if ui.button("Add default node").clicked()
            {
                studio_context.graph_editor.add_node("addition", egui::Pos2 { x: 0.0, y: 0.0 });
            }
        });

        if ui.button("Toggle debug window").clicked()
        {
            studio_context.layout.debug_window_active = !studio_context.layout.debug_window_active;
        }
    });

    if studio_context.executor.is_none()
    {
        if ui.button("▶ Start").clicked()
        {
            let mut empower_executor = EmpowerExecutor::new(studio_context.graph_editor.node_graph.clone(), false);
            empower_executor.start_node_graph();

            studio_context.executor = Some( empower_executor );
        }
    }
    else 
    {
        ui.horizontal(|ui|
        {
            if ui.button("⏹ Stop").clicked()
            {
                studio_context.executor = None;
            }
            ui.spinner(); 
        });
    }


    ui.add_space(2.0);

}