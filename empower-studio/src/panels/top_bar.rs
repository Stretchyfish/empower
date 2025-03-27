use egui;
use egui_dock;

use crate::panels;
use crate::viewports;
use crate::EmpowerEditorState;
use crate::NodeGraph;

pub fn view_menu_bar(ui: &mut egui::Ui, empower_editor_state: &mut EmpowerEditorState, workspace: &mut panels::Workspace, docking_state: &mut egui_dock::DockState<String>, node_graph: &mut NodeGraph)
{
    egui::menu::bar(ui, |ui|
    {
        ui.menu_button("Nodes", |ui|
        {
            if ui.button("Add default node").clicked()
            {
                node_graph.add_node();
            }
        });

        if ui.button("Compile nodes").clicked()
        {
            //self.engine.compile();
            // context.graph_state.engine.compile();
            // context.graph_state.refresh_input_port_values();         
            println!("compiling");
        }

        if ui.button("turn on debug mode").clicked()
        {
            empower_editor_state.debug_panel_active = !empower_editor_state.debug_panel_active;
        }

        ui.menu_button("add panel", |ui|
        {
            if ui.button("Add graph viewport").clicked()
            {
                let new_viewport_type = viewports::ViewportTypes::GraphViewport;
                let new_viewport_name = workspace.create_viewport(new_viewport_type);
                docking_state.push_to_focused_leaf(new_viewport_name); 
            };
            if ui.button("Add empty viewport").clicked()
            {
                let new_viewport_type = viewports::ViewportTypes::Empty;
                let new_viewport_name = workspace.create_viewport(new_viewport_type);
                docking_state.push_to_focused_leaf(new_viewport_name); 
            };
        });
    });
}
