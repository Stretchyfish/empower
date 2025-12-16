use empower_engine::runtime::EmpowerExecutor;

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

        ui.menu_button("Add viewport", |ui|
        {
            if ui.button("Graph viewport").clicked()
            {
                studio_context.layout.add_viewport("graph viewport");
            }
            if ui.button("Terminal Viewport").clicked()
            {
                studio_context.layout.add_viewport("terminal viewport");
            }
            if ui.button("Empty Viewport").clicked()
            {
                studio_context.layout.add_viewport("empty viewport");
            }
        });
    });

    if studio_context.graph_editor.executor.is_none()
    {
        if ui.button("▶ Start").clicked()
        {
            let mut empower_executor = EmpowerExecutor::new(studio_context.graph_editor.node_graph.clone(), true, true);
            empower_executor.start_node_graph();

            studio_context.graph_editor.executor = Some( empower_executor );
        }
    }
    else 
    {
        ui.horizontal(|ui|
        {
            if ui.button("⏹ Stop").clicked()
            {
                studio_context.graph_editor.executor = None;
            }
            ui.spinner(); 
        });
    }


    ui.add_space(2.0);

}