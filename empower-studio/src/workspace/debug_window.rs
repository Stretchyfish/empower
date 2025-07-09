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

                ui.label(format!("Input ports: {}", studio_context.graph_editor.empower_node_graph.input_ports.len()));
                ui.label(format!("Output ports: {}", studio_context.graph_editor.empower_node_graph.output_ports.len()));
                ui.label(format!("Connections out: {}", studio_context.graph_editor.empower_node_graph.connections_out.len()));
                ui.label(format!("Connections in: {}", studio_context.graph_editor.empower_node_graph.connections_in.len()));

                egui::CollapsingHeader::new(format!("Nodes: {}", studio_context.graph_editor.empower_node_graph.nodes.len()))
                    .default_open(false)
                    .show(ui, |ui| 
                    {
                        ui.horizontal(|ui|
                        {
                            ui.label("id");
                            ui.label("type");
                            ui.label("input ports");
                            ui.label("output ports");
                        });
                        for (_, node) in studio_context.graph_editor.empower_node_graph.nodes.iter()
                        {
                            let input_port_keys = &node.input_port_keys;
                            let output_port_keys = &node.output_port_keys;

                            ui.horizontal(|ui|
                            {
                                ui.vertical(|ui|
                                {
                                    ui.label(node.key.to_string());
                                });

                                ui.vertical(|ui|
                                {
                                    ui.label(node.node_type.to_string());
                                });

                                ui.vertical(|ui|
                                {
                                    for input_port_key in input_port_keys
                                    {
                                        ui.label(input_port_key.to_string());
                                    }
                                });

                                ui.vertical(|ui|
                                {
                                    for output_port_key in output_port_keys
                                    {
                                        ui.label(output_port_key.to_string());
                                    }
                                });
                            });
                        }
                    });
                    egui::CollapsingHeader::new(format!("Input ports: {}", studio_context.graph_editor.empower_node_graph.input_ports.len()))
                        .default_open(false)
                        .show(ui, |ui| 
                    {
                        ui.horizontal(|ui|
                        {
                            ui.label("id");
                            ui.label("type");
                            ui.label("value");
                       });
                
                        for (key, port) in studio_context.graph_editor.empower_node_graph.input_ports.iter()
                        {
                            ui.horizontal(|ui|
                            {
                                ui.label(key.to_string());
                                ui.label(port.value.get_type());
                                ui.label(port.value.to_string());
                           });

                        }
                    });
                    egui::CollapsingHeader::new(format!("Output ports: {}", studio_context.graph_editor.empower_node_graph.output_ports.len()))
                        .default_open(false)
                        .show(ui, |ui| 
                    {
                        ui.horizontal(|ui|
                        {
                            ui.label("id");
                            ui.label("type");
                            ui.label("value");
                       });
                
                        for (key, port) in studio_context.graph_editor.empower_node_graph.output_ports.iter()
                        {
                            ui.horizontal(|ui|
                            {
                                ui.label(key.to_string());
                                ui.label(port.value.get_type());
                                ui.label(port.value.to_string());
                            });
                        }
                    });

                    egui::CollapsingHeader::new(format!("Connections: {}", studio_context.graph_editor.empower_node_graph.connections_out.len()))
                        .default_open(false)
                        .show(ui, |ui| 
                    {
                        ui.horizontal(|ui|
                        {
                            ui.label("port out");
                            ui.label("port in");
                        });
                        for (key, connected_input_ports) in studio_context.graph_editor.empower_node_graph.connections_out.iter()
                        {
                            ui.horizontal(|ui|
                            {
                                ui.label(key.to_string());

                                ui.vertical(|ui|
                                {
                                    for input_port in connected_input_ports
                                    {
                                        ui.label(input_port.to_string());
                                    }
                                });
                            });
                        }
                    });
                });

                ui.add_space(0.5);
                ui.vertical(|ui| {
                    ui.heading("Display Graph");
                    egui::CollapsingHeader::new(format!("Display Nodes: {}", studio_context.graph_editor.display_nodes.len()))
                        .default_open(false)
                        .show(ui, |ui| {

                            
                            ui.horizontal(|ui|
                            {
                                ui.label("id");
                            });
                            for (key, _) in studio_context.graph_editor.display_nodes.iter()
                            {
                                ui.horizontal(|ui|
                                {
                                    ui.vertical(|ui|
                                    {
                                        ui.label(key.to_string());
                                    });

                                });
                            }
                        });

                    egui::CollapsingHeader::new(format!("Display Input Ports: {}", studio_context.graph_editor.display_input_ports.len()))
                    .default_open(false)
                    .show(ui, |ui|
                    {
                            ui.horizontal(|ui|
                            {
                                ui.label("id");
                            });
                            for (key, _) in studio_context.graph_editor.display_input_ports.iter()
                            {
                                ui.horizontal(|ui|
                                {
                                    ui.vertical(|ui|
                                    {
                                        ui.label(key.to_string());
                                    });
                                });
                            }
                    });

                    egui::CollapsingHeader::new(format!("Display Output Ports: {}", studio_context.graph_editor.display_output_ports.len()))
                    .default_open(false)
                    .show(ui, |ui|
                    {
                            ui.horizontal(|ui|
                            {
                                ui.label("id");
                            });
                            for (key, _) in studio_context.graph_editor.display_output_ports.iter()
                            {
                                ui.horizontal(|ui|
                                {
                                    ui.vertical(|ui|
                                    {
                                        ui.label(key.to_string());
                                    });
                                });
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
                                
                                ui.label("Scene mouse position");
                                ui.label(format!("{},{}", graph_viewport.mouse_scene_position_last_frame.x, graph_viewport.mouse_scene_position_last_frame.y));

                                ui.label("Delta mouse position");
                                ui.label(format!("{},{}", graph_viewport.mouse_delta_last_frame.x, graph_viewport.mouse_delta_last_frame.y));
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