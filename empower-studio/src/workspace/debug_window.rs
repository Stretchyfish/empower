// use empower_engine::node_graph;

use crate::studio_context::StudioContext;

pub fn show(ctx: &egui::Context, studio_context: &mut StudioContext)
{
    egui::Window::new("Debug Panel")
    .collapsible(true)
    .resizable(false)
    .open(&mut studio_context.layout.debug_window_active)
    .show(ctx, |ui| 
    {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.heading("Empower Node Graph");

                ui.label(format!("Input ports: {}", studio_context.graph_editor.node_graph.input_port_count()));
                ui.label(format!("Output ports: {}", studio_context.graph_editor.node_graph.output_port_count()));
                ui.label(format!("Connections: {}", studio_context.graph_editor.node_graph.connections_count()));

                egui::CollapsingHeader::new(format!("Nodes: {}", studio_context.graph_editor.node_graph.node_count()))
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
                        for node in studio_context.graph_editor.node_graph.get_all_nodes()
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
                                    ui.label(node.kind.name().to_string());
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
                    egui::CollapsingHeader::new(format!("Input ports: {}", studio_context.graph_editor.node_graph.input_port_count()))
                        .default_open(false)
                        .show(ui, |ui| 
                    {
                        ui.horizontal(|ui|
                        {
                            ui.label("id");
                            ui.label("type");
                            ui.label("value");
                       });
                
                        for port in studio_context.graph_editor.node_graph.get_all_input_ports()
                        {
                            ui.horizontal(|ui|
                            {
                                ui.label(port.key.to_string());
                                ui.label(port.value.type_name());
                                ui.label(port.value.to_string());
                           });

                        }
                    });
                    egui::CollapsingHeader::new(format!("Output ports: {}", studio_context.graph_editor.node_graph.output_port_count()))
                        .default_open(false)
                        .show(ui, |ui| 
                    {
                        ui.horizontal(|ui|
                        {
                            ui.label("id");
                            ui.label("type");
                            ui.label("value");
                       });
                
                        for port in studio_context.graph_editor.node_graph.get_all_output_ports()
                        {
                            ui.horizontal(|ui|
                            {
                                ui.label(port.key.to_string());
                                ui.label(port.value.type_name());
                                ui.label(port.value.to_string());
                            });
                        }
                    });

                    egui::CollapsingHeader::new(format!("Connections: {}", studio_context.graph_editor.node_graph.connections_count()))
                        .default_open(false)
                        .show(ui, |ui| 
                    {
                        // ui.horizontal(|ui|
                        // {
                        //     ui.label("port out");
                        //     ui.label("port in");
                        // });
                        for connection in studio_context.graph_editor.node_graph.get_all_connections()
                        {
                            ui.horizontal(|ui|
                            {
                                ui.label(format!("in: {}, out: {}", connection.0.to_string(), connection.1.to_string()));

                                // ui.vertical(|ui|
                                // {
                                //     for input_port in connected_input_ports
                                //     {
                                //         ui.label(input_port.to_string());
                                //     }
                                // });
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

                    egui::CollapsingHeader::new(format!("Selected nodes: {}", studio_context.graph_editor.selected_nodes.len()))
                    .default_open(false)
                    .show(ui, |ui|
                    {
                            ui.horizontal(|ui|
                            {
                                ui.label("id");
                            });
                            for key in studio_context.graph_editor.selected_nodes.iter()
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

                // ui.vertical(|ui| {
                //     ui.heading("Viewports");

                //     for graph_viewport in studio_context.workspace.viewports.graph_viewports.iter()
                //     {
                //         egui::CollapsingHeader::new(graph_viewport.title.clone())
                //             .default_open(false)
                //             .show(ui, |ui| {
                                
                //                 ui.label("Scene mouse position");
                //                 ui.label(format!("{},{}", graph_viewport.mouse_scene_position_last_frame.x, graph_viewport.mouse_scene_position_last_frame.y));

                //                 ui.label("Delta mouse position");
                //                 ui.label(format!("{},{}", graph_viewport.mouse_delta_last_frame.x, graph_viewport.mouse_delta_last_frame.y));
                //             });
                //     } 
                //     for empty_viewport in studio_context.workspace.viewports.empty_viewports.iter()
                //     {
                //         ui.label(empty_viewport.title.clone());
                //     } 
                // });
                ui.add_space(0.5);
            });
    
        ui.horizontal(|ui| 
        {
            ui.checkbox(&mut studio_context.graph_editor.debug_info.show_node_execution_order, "Show node execution order");

            if ui.button("Refresh execution order").clicked()
            {
                let node_execution_order = empower_engine::runtime::analysis::detect_execution_order(&mut studio_context.graph_editor.node_graph);
                studio_context.graph_editor.debug_info.node_execution_order = node_execution_order;
            }
        });
    
    });
}