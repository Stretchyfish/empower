use std::path::PathBuf;

use chrono::Local;
use empower_engine::{NodeGraph, utility::text_buffer::TextBuffer};

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
                action_queue.push( Action::SaveProject );
            }
            if ui.button("Save As").clicked()
            {
                action_queue.push( Action::SaveProjectAs );
            }
            if ui.button("Open").clicked()
            {
                let project_path = rfd::FileDialog::new()
                                                    .set_title("Choose project location")
                                                    .set_can_create_directories(true)
                                                    .pick_folder();
                action_queue.push( Action::LoadProject { project_path: project_path.unwrap() } ); // @TODO, this is dangerous
            }
            ui.menu_button("Open Recent", |ui|
            {
                for previous_project in &studio_context.layout.previous_projects
                {
                    if ui.button(previous_project.project_name.to_string()).clicked()
                    {

                        action_queue.push( Action::LoadProject { project_path: previous_project.project_location.clone() } ); // @TODO, this is dangerous
                        
                    }
                }
            });
            if ui.button("Project Settings").clicked()
            {
                studio_context.layout.project_setting_window = !studio_context.layout.project_setting_window;
            }
            if ui.button("Export").clicked()
            {
                
            }
        });
        
        if ui.button("Toggle debug window").clicked()
        {
            action_queue.push( Action::ToggleDebugWindow );
        }

        ui.menu_button("Windows", |ui|
        {
            if ui.button("default layout").clicked()
            {
                action_queue.push( Action::DefaultLayout );
            }
            if ui.button("clear layout").clicked()
            {
                action_queue.push( Action::ClearLayout );
            }
            if ui.button("Graph viewport").clicked()
            {
                action_queue.push( Action::CreateViewport { name: "graph viewport" });
            }
            if ui.button("Terminal Viewport").clicked()
            {
                action_queue.push( Action::CreateViewport { name: "terminal viewport" });
            }
            if ui.button("Content Browser Viewport").clicked()
            {
                action_queue.push( Action::CreateViewport { name: "content browser viewport" });
            }
            if ui.button("Empty Viewport").clicked()
            {
                action_queue.push( Action::CreateViewport { name: "empty viewport" });
            }

            if ui.button("save editor state").clicked()
            {
                action_queue.push( Action::SaveEditorState );
            }

            if ui.button("load editor state").clicked()
            {
                action_queue.push( Action::LoadEditorState );
            }
        });


        // if ui.button("test save node graph").clicked()
        // {
        //     let path_to_desktop = PathBuf::from("/home/mikkel/Desktop/graph_editor_save.json");
        //     studio_context.project.graph_editor.save(path_to_desktop);
        // }

        // if ui.button("test load node graph").clicked()
        // {
        //     let path_to_desktop = PathBuf::from("/home/mikkel/Desktop/graph_editor_save.json");
        //     studio_context.project.graph_editor = crate::graph_editor::GraphEditor::load(path_to_desktop);
            
        // }
    });

    ui.horizontal(|ui|
    {
        if studio_context.project.graph_editor.executor.is_none()
        {
            ui.scope(|ui|
            {
                ui.style_mut().visuals.widgets.inactive.weak_bg_fill = egui::Color32::DARK_GREEN;
                ui.style_mut().visuals.widgets.hovered.weak_bg_fill = egui::Color32::LIGHT_GREEN;
                if ui.button("▶ Start").clicked()
                {
                    action_queue.push( Action::StartNodeGraphExecution );
                    studio_context.project.graph_editor.executor_history = Some( (Local::now(), TextBuffer::new()) ); // @TODO, make this behavior better
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
                        let executor = studio_context.project.graph_editor.executor.as_ref().unwrap(); // @TODO, find a better way to achieve this behavior
                        studio_context.project.graph_editor.executor_history = Some( ( executor.start_time.clone(), executor.history.clone() )); // @TODO, make a shared way to save this independent of where its stopped
                    }
                }
                ui.spinner(); 
            });
        }

        ui.checkbox(&mut studio_context.project.graph_editor.debug_info.show_keys, "show keys");

        ui.add_space(20.0);

        if studio_context.project.graph_editor.executor_history.is_some()
        {
            let execution_history = studio_context.project.graph_editor.executor_history.as_ref().unwrap();

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
