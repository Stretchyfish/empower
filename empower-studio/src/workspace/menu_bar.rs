use crate::{StudioContext, actions::Action};

pub fn show(ui: &mut egui::Ui, studio_context: &StudioContext, action_queue: &mut Vec<Action>)
{
    egui::MenuBar::new()
    .ui(ui, |ui|
    {
        ui.menu_button("Nodes", |ui|
        {
            if ui.button("Add test node").clicked()
            {
                action_queue.push( Action::CreateNode { name: "vector", position: egui::Pos2::ZERO } );
            }
        });

        if ui.button("Toggle debug window").clicked()
        {
            action_queue.push( Action::ToggleDebugWindow );
        }

        ui.menu_button("Add viewport", |ui|
        {
            if ui.button("Graph viewport").clicked()
            {
                action_queue.push( Action::CreateViewport { name: "graph viewport" });
            }
            if ui.button("Terminal Viewport").clicked()
            {
                action_queue.push( Action::CreateViewport { name: "terminal viewport" });
            }
            if ui.button("Empty Viewport").clicked()
            {
                action_queue.push( Action::CreateViewport { name: "empty viewport" });
            }
        });
    });

    if studio_context.graph_editor.executor.is_none()
    {
        if ui.button("▶ Start").clicked()
        {
            action_queue.push( Action::StartNodeGraphExecution );
        }
    }
    else 
    {
        ui.horizontal(|ui|
        {
            if ui.button("⏹ Stop").clicked()
            {
                action_queue.push( Action::StopNodeGraphExecution );
            }
            ui.spinner(); 
        });
    }


    ui.add_space(2.0);

}
