use chrono::Local;
use empower_engine::utility::text_buffer::TextBuffer;

use crate::{StudioContext, actions::Action};

// @TODO, change studio context back to being const
pub fn show(ui: &mut egui::Ui, studio_context: &mut StudioContext, action_queue: &mut Vec<Action>)
{
    egui::MenuBar::new()
    .ui(ui, |ui|
    {
        ui.menu_button("Project", |ui|
        {
            if ui.button("Save").clicked()
            {
                let file_path = rfd::FileDialog::new().pick_file();

                if file_path.is_some()
                {
                    let file_path  = file_path.unwrap().to_str().unwrap().to_string();

                    action_queue.push( Action::SaveProject { project_name: file_path });
                }
            }
            if ui.button("Save As").clicked()
            {
            }
            if ui.button("Open").clicked()
            {
            }
            if ui.button("Open Recent").clicked()
            {
            }
        });
        
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

    ui.horizontal(|ui|
    {
        if studio_context.graph_editor.executor.is_none()
        {
            ui.scope(|ui|
            {
                ui.style_mut().visuals.widgets.inactive.weak_bg_fill = egui::Color32::DARK_GREEN;
                ui.style_mut().visuals.widgets.hovered.weak_bg_fill = egui::Color32::LIGHT_GREEN;
                if ui.button("▶ Start").clicked()
                {
                    action_queue.push( Action::StartNodeGraphExecution );
                    studio_context.graph_editor.executor_history = Some( (Local::now(), TextBuffer::new()) ); // @TODO, make this behavior better
                }
            });


            
        }
        else 
        {
            ui.scope(|ui|
            {
                ui.style_mut().visuals.widgets.inactive.weak_bg_fill = egui::Color32::DARK_RED;
                ui.style_mut().visuals.widgets.hovered.weak_bg_fill = egui::Color32::LIGHT_RED;
                if ui.button("⏹ Stop").clicked()
                {
                    action_queue.push( Action::StopNodeGraphExecution );
                    {
                        let executor = studio_context.graph_editor.executor.as_ref().unwrap(); // @TODO, find a better way to achieve this behavior
                        studio_context.graph_editor.executor_history = Some( ( executor.start_time.clone(), executor.history.clone() )); // @TODO, make a shared way to save this independent of where its stopped
                    }
                }
                ui.spinner(); 
            });
        }

        ui.checkbox(&mut studio_context.graph_editor.debug_info.show_keys, "show keys");

        ui.add_space(20.0);

        if studio_context.graph_editor.executor_history.is_some()
        {
            let execution_history = studio_context.graph_editor.executor_history.as_ref().unwrap();

            let time_as_text = format!("{}", execution_history.0.format("%Y-%m-%d %H:%M:%S"));
            ui.label(time_as_text);

            if ui.button("show execution history").clicked()
            {
                action_queue.push( Action::ToggleExecutionHisotryWindow );
            }
        }
    });

    ui.add_space(2.0);
}
