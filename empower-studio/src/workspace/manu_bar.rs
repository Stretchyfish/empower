use crate::{graph_editor, studio_context::StudioContext, workspace::viewports::viewport_type::ViewportType};

pub fn show(ui: &mut egui::Ui, studio_context: &mut StudioContext)
{
    egui::menu::bar(ui, |ui|
    {
        ui.menu_button("Nodes", |ui|
        {
            if ui.button("Add default node").clicked()
            {

            }
        });

        ui.menu_button("test setups", |ui| 
        {
            if ui.button("setup one connection").clicked()
            {

            }
        });

        if ui.button("Compile nodes").clicked()
        {
            // @TODO, condier adding a check, that all display port values are valid?

            empower_engine::debug_compile(&mut studio_context.graph_editor.empower_node_graph);
            println!("compiling");

            studio_context.graph_editor.refresh_display_port_values();
        }

        if ui.button("turn on debug mode").clicked()
        {
        }

        ui.menu_button("add panel", |ui|
        {
            if ui.button("Add graph viewport").clicked()
            {
                let new_viewport_type = ViewportType::GraphViewport;
                studio_context.workspace.add_viewport(new_viewport_type);
            };
            if ui.button("Add empty viewport").clicked()
            {
                let new_viewport_type = ViewportType::EmptyViewport;
                studio_context.workspace.add_viewport(new_viewport_type);
            };
        });
    });
}
