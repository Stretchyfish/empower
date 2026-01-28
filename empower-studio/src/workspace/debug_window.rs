use empower_engine::node_graph::node::port::PortKind;

use crate::studio_context::StudioContext;

pub fn show(ctx: &egui::Context, studio_context: &mut StudioContext)
{
    let mut window_active = studio_context.layout.debug_window_active;
    egui::Window::new("Debug Panel")
    .collapsible(true)
    .resizable(false)
    .open(&mut window_active)
    .show(ctx, |ui| 
    {
        ui.horizontal(|ui| {
            node_graph_debugging(ui, studio_context);

            ui.add_space(0.5);

            display_graph_debugging(ui, studio_context);

            ui.add_space(0.5);

            executeion_debugging(ui, studio_context);
        });
    
        ui.horizontal(|ui| 
        {
            ui.checkbox(&mut studio_context.graph_editor.debug_info.show_node_execution_order, "Show node execution order");

            if ui.button("Refresh execution order").clicked()
            {
                // let node_execution_order = empower_engine::runtime::analysis::detect_execution_order(&mut studio_context.graph_editor.node_graph);
                let start_node_key = 1; // @TODO, find a better way of doing this?
                let node_execution_order = empower_engine::runtime::analysis::detect_execution_order( &start_node_key, &mut studio_context.graph_editor.node_graph);
                studio_context.graph_editor.debug_info.node_execution_order = node_execution_order;
            }
        });
    
    });

    studio_context.layout.debug_window_active = window_active;
}

fn node_graph_debugging(ui: &mut egui::Ui, studio_context: &mut StudioContext)
{
    ui.vertical(|ui|
    {
        ui.heading("Node Graph");

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
}

fn display_graph_debugging(ui: &mut egui::Ui, studio_context: &mut StudioContext)
{
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

        let port_searcher_show =
        {
            match studio_context.graph_editor.port_searcher
            {
                Some(_) => "active",
                None => "inactive",
            }
        };

        egui::CollapsingHeader::new(format!("Port Searcher: {}", port_searcher_show))
        .default_open(true)
        .show(ui, |ui|
        {
            if studio_context.graph_editor.port_searcher.is_none()
            {
                return;
            }

            let port_searcher = studio_context.graph_editor.port_searcher.as_ref().unwrap();
            ui.horizontal(|ui|
            {
                ui.label(format!("Key: {}", port_searcher.port_key));

                let port_kind_text =
                {
                    match port_searcher.port_kind
                    {
                        PortKind::Input => "input",
                        PortKind::Output => "output",
                    }
                };

                ui.label(format!("Kind: {}", port_kind_text));
            });
        });
    });
}

fn executeion_debugging(ui: &mut egui::Ui, studio_context: &mut StudioContext)
{
    ui.vertical(|ui| {

        let mut active_text = "inactive";
        if studio_context.graph_editor.executor.is_some()
        {
            active_text = "active";
        }


        egui::CollapsingHeader::new(format!("Executor ({})", active_text))
        .default_open(false)
        .show(ui, |ui| {

            if studio_context.graph_editor.executor.is_none()
            {
                return;
            }

            let executor = studio_context.graph_editor.executor.as_ref().unwrap();

            egui::CollapsingHeader::new(format!("Task Manager ({})", executor.task_manager.tasks.len()))
            .default_open(false)
            .show(ui, |ui| {

                for (task_id, task) in &executor.task_manager.tasks
                {

                    ui.label(format!("Task id: {}", task_id.to_string()));
                    ui.label(format!("Setup keys: {:?}", task.nodes_to_setup));
                    ui.label(format!("Update keys: {:?}", task.nodes_to_update));
                }
            });
            egui::CollapsingHeader::new("Window Manager")
            .default_open(false)
            .show(ui, |ui| {

                ui.horizontal(|ui|
                {
                    ui.label("task id");
                    ui.label("node key");
                    ui.label("window name");                    
                });
                
                // for (task_id, windows) in executor.task_manager.windows.iter()
                // {
                //     for (node_key, window_name) in windows
                //     {
                //         ui.horizontal(|ui|
                //         {
                //             ui.label(task_id.to_string());
                //             ui.label(node_key.to_string());
                //             ui.label(window_name);
                //         });
                //     }
                // }
            });

            let cached_output_port_text = format!("Cached Output Ports ({})", executor.cached_output_ports.len());

            egui::CollapsingHeader::new(cached_output_port_text)
            .default_open(false)
            .show(ui, |ui| {

                for port_key in executor.cached_output_ports.iter()
                {
                    ui.label(format!("{}", port_key));
                }
            });

            let executed_nodes_history = format!("Executed nodes history ({})", executor.history.lines.len());

            egui::CollapsingHeader::new(executed_nodes_history)
            .default_open(false)
            .show(ui, |ui| {

                for line in executor.history.lines.iter()
                {
                    ui.label(format!("{}", line));
                }
            });

            ui.label(format!("Last executed node: {}", executor.last_executed_node));

            ui.label("Relations");

                for (node_key, tasks) in &executor.task_manager.relations
                {
                    ui.horizontal(|ui|
                    {
                        ui.label(format!("node: {}", node_key));
                        ui.label(format!("tasks: {:?}", tasks));
                    });
                }
        });
    });
}
