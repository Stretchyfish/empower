use crate::studio_context::StudioContext;

pub fn show(ctx: &egui::Context, studio_context: &mut StudioContext)
{
egui::Window::new("Debug Panel")
    .collapsible(true)
    .resizable(false)
    .open(&mut studio_context.workspace.debug_window_active)
    .show(ctx, |ui| 
    {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.heading("Empower Node Graph");
                egui::CollapsingHeader::new(format!("Nodes: {}", studio_context.graph_editor.empower_node_graph.nodes.len()))
                    .default_open(false)
                    .show(ui, |ui| 
                    {
                        ui.label("| id | type |"); // @TODO, find way of doing horizontal setup
                        for node in studio_context.graph_editor.empower_node_graph.nodes.iter()
                        {
                         ui.label(format!("{}, {}", node.0, "int"));
                        }
                        
                    });
                ui.label(format!("Input ports: {}", studio_context.graph_editor.empower_node_graph.input_ports.len()));
                ui.label(format!("Output ports: {}", studio_context.graph_editor.empower_node_graph.output_ports.len()));
                ui.label(format!("Connections out: {}", studio_context.graph_editor.empower_node_graph.connections_out.len()));
                ui.label(format!("Connections in: {}", studio_context.graph_editor.empower_node_graph.connections_in.len()));
           });
                ui.add_space(0.5);

                ui.vertical(|ui| {
                    ui.heading("Display Graph");
                    egui::CollapsingHeader::new(format!("Display Nodes: {}", studio_context.graph_editor.display_nodes.len()))
                        .default_open(false)
                        .show(ui, |ui| {
                            
                            ui.label("| id | type |");
                            for (display_node_key, _) in studio_context.graph_editor.display_nodes.iter()
                            {
                                ui.label(format!("{}, {}", display_node_key, "int"));
                            }
                        });
                });
                ui.add_space(0.5);

                ui.vertical(|ui| {
                    ui.heading("Viewports");

                    for graph_viewport in studio_context.workspace.viewports.graph_viewports.iter()
                    {
                        egui::CollapsingHeader::new(graph_viewport.title.clone())
                            .default_open(false)
                            .show(ui, |ui| {
                                
                            });
                    } 
                    for empty_viewport in studio_context.workspace.viewports.empty_viewports.iter()
                    {
                        ui.label(empty_viewport.title.clone());
                    } 
                });
                ui.add_space(0.5);
            });
        });
}