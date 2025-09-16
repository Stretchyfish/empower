use empower_engine::{analyser, node_graph};

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
            // @TODO, combine these
            studio_context.graph_editor.node_graph.execute_node_graph();
            studio_context.graph_editor.refresh_all_node_display();
            // @TODO, condier adding a check, that all display port values are valid?

            // let empower_result = empower_engine::debug_compile(&mut studio_context.graph_editor.empower_node_graph);

            // // @TODO, find a better way to do this!
            // for terminal_viewport in studio_context.workspace.viewports.terminal_viewports.iter_mut()
            // {
            //     for line in empower_result.get_lines()
            //     {
            //         terminal_viewport.add_line(line);
            //     }
            //     terminal_viewport.add_line(String::from(format!("Compilation time: {} sec", empower_result.compilation_duration)));
            // }

            // studio_context.graph_editor.refresh_display_port_values();
        }

        if ui.button("turn on debug mode").clicked()
        {
            studio_context.workspace.debug_window_active = !studio_context.workspace.debug_window_active;
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
            if ui.button("Add terminal viewport").clicked()
            {
                let new_viewport_type = ViewportType::TerminalViewport;
                studio_context.workspace.add_viewport(new_viewport_type);
            }
        });

    });
}
