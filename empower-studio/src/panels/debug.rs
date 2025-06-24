use crate::panels;
use crate::EmpowerEditorState;
use crate::StudioContext;
use egui;

pub fn show_debug_panel(
    ctx: &egui::Context,
    empower_editor_state: &mut EmpowerEditorState,
    workspace: &mut panels::Workspace,
    studio_context: &mut StudioContext
) {
    egui::Window::new("Debug Panel")
    .collapsible(true)
    .resizable(false)
    .open(&mut empower_editor_state.debug_panel_active)
    .show(ctx, |ui| 
    {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.heading("Engine state");
                egui::CollapsingHeader::new(format!("Nodes: {}", studio_context.empower_node_graph.nodes.len()))
                    .default_open(false)
                    .show(ui, |ui| 
                    {

                        ui.label("| id | type |");
                        for empower_node in studio_context.empower_node_graph.nodes.iter() // @TODO, simplify this call
                        {
                         ui.label(format!("{}, {}", empower_node.0, "int"));
                        }
                        
                    });
                // ui.label(format!("Input ports: {}", studio_context.empower_node_graph.input_ports.len()));
                // ui.label(format!("Output ports: {}", studio_context.empower_node_graph.output_ports.len()));
            });
                ui.add_space(0.5);

                ui.vertical(|ui| {
                    ui.heading("Display Graph");
                    egui::CollapsingHeader::new(format!("Display Nodes: {}", studio_context.display_node_graph.display_nodes.len()))
                        .default_open(false)
                        .show(ui, |ui| {
                            
                            ui.label("| id | type |");
                            for (display_node_key, _) in studio_context.display_node_graph.display_nodes.iter()
                            {
                                ui.label(format!("{}, {}", display_node_key, "int"));
                            }
                        });
                    // ui.label(format!("Nodes: {}", node_graph.nodes.len()));
                });
                ui.add_space(0.5);

                ui.vertical(|ui| {
                    ui.heading("Viewports");

                    for graph_viewport in workspace.graph_viewports.iter()
                    {
                        egui::CollapsingHeader::new(graph_viewport.title.clone())
                            .default_open(false)
                            .show(ui, |ui| {
                                
                                ui.label("Selected nodes: ".to_string() + graph_viewport.state.selected_nodes.len().to_string().as_str());
                                // ui.label("Pan zoom");
                                // ui.label(format!(" - Pan offset: {},{}", graph_viewport.state.pan_zoom.pan_offset.x, graph_viewport.state.pan_zoom.pan_offset.y));
                                // ui.label(format!(" - Zoom scale: {}", graph_viewport.state.pan_zoom.zoom_scale));
                            });
                    } 
                    for empty_viewport in workspace.empty_viewports.iter()
                    {
                        ui.label(empty_viewport.title.clone());
                    } 
                });
                ui.add_space(0.5);

            });
        });
}
