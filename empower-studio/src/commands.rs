use crate::{studio_context::layout::Layout, studio_context::project::{Project, ProjectState}};

pub enum Command
{
    None,
    SetDefaultLayout,
    SetClearLayout,
    AddViewport { name: &'static str },
    SaveLayout, 
    LoadLayout,
    SaveProject,
    SaveProjectAs,
    OpenProject,
}

// pub fn process_command(command: Command, layout: &mut Layout, project: &mut Project, user_state: &mut UserState)
// {
    // match user_state
    // {
    //     UserState::Idle => {},
    //     UserState::DraggingAsset => {},
    //     UserState::NamingProject { new_project_name: _ } => { return; },
    // };
    
    // match command
    // {
    //     Command::None => {},
    //     Command::SetDefaultLayout => { *layout = Layout::default_layout(); },
    //     Command::SetClearLayout => { *layout = Layout::new() },
    //     Command::AddViewport { name } => { layout.add_viewport(name); },
    //     Command::SaveLayout => { layout.save(); },
    //     Command::LoadLayout => { *layout = Layout::load(); },
    //     Command::SaveProject => 
    //     { 
    //         if project.state == ProjectState::Temporary && project.name == String::from("untitled") // @TODO, not a great way of handling this
    //         {
    //             *user_state = UserState::NamingProject { new_project_name: project.name.clone() };
    //             return;
    //         }
    //         project.save(); 
    //     },
    //     Command::SaveProjectAs => { project.save_as(); },
    //     Command::OpenProject => { },
    // };
// }
    // pub fn process_actions(&mut self, ctx: &egui::Context, action_queue: Vec<Action>)
    // {
    //     for action in action_queue
    //     {
    //         match &action
    //         {
    //             Action::CreateTemporaryProject =>
    //             {
    //                 self.project.new_project();
    //             },
    //             Action::SaveProject => 
    //             { 
    //                 match self.project.state.clone()
    //                 {
    //                     ProjectState::Undefined => todo!(),
    //                     ProjectState::Temporary(_) => 
    //                     {
    //                         // self.layout.project_name_window = Some( self.project.name.clone() );
    //                         return;
    //                     },
    //                     ProjectState::Saved( project_path ) => 
    //                     {
    //                         self.project.save();
    //                         // self.layout.add_previous_project(&self.project.name, &project_path);
    //                     },
    //                 }
    //                 ctx.send_viewport_cmd(egui::ViewportCommand::Title(format!("empower studio - {}", self.project.name)));
    //             },
    //             Action::SaveProjectAs =>
    //             {
    //                 let project_state = self.project.state.clone();
    //                 match project_state
    //                 {
    //                     ProjectState::Undefined => todo!(),
    //                     ProjectState::Temporary(_) => 
    //                     {
    //                         // self.layout.project_name_window = Some( self.project.name.clone() );
    //                         return;
    //                     },
    //                     ProjectState::Saved( path_buf ) =>
    //                     {
    //                         self.project.save_as();
    //                         // self.layout.add_previous_project(&self.project.name, &path_buf);
    //                     }
    //                 }
    //                 ctx.send_viewport_cmd(egui::ViewportCommand::Title(format!("empower studio - {}", self.project.name)));
    //             },
    //             Action::LoadProject { project_path } =>
    //             {
    //                 // self.project = Project::load(project_path);

    //                 // let project_path = match &self.project.state // @TODO, this is dangerous, needs to be dealth with
    //                 // {
    //                 //     ProjectState::Undefined => todo!(),
    //                 //     ProjectState::Temporary(_) => todo!(),
    //                 //     ProjectState::Saved(path_buf) => path_buf,
    //                 // };
                    
    //                 // self.layout.add_previous_project(&self.project.name, &project_path);
    //                 // ctx.send_viewport_cmd(egui::ViewportCommand::Title(format!("empower studio - {}", self.project.name)));
    //             },
    //             Action::CreateNode { name, position } => { self.project.graph_editor.add_node(name, *position); },
    //             Action::RefreshNodeStructure { node_key } => { self.project.graph_editor.refresh_node_strcuture(&node_key); },
    //             Action::DeleteNode { node_key } => { self.project.graph_editor.remove_node(&node_key); },
    //             Action::CopySelectedNodes =>
    //                         {
    //                             for node_key in self.project.graph_editor.selected_nodes.clone()
    //                             {
    //                                 self.project.graph_editor.toggle_node_selection(&node_key);
    //                                 let copied_node_key = self.project.graph_editor.create_node_copy(&node_key);
    //                                 self.project.graph_editor.toggle_node_selection(&copied_node_key);
    //                             }
    //                         },
    //             Action::ToggleNodeSelection { node_key } => self.project.graph_editor.toggle_node_selection(&node_key),
    //             Action::AddNodesToSelectedNodes { node_keys } => for node_key in node_keys { self.project.graph_editor.add_node_to_selection(&node_key); },
    //             Action::ClearAllNodesFromSelectedNodes => self.project.graph_editor.clear_node_selection(),
    //             Action::MoveSelectedNodes { canvas_delta_position } => self.project.graph_editor.move_selected_nodes(&canvas_delta_position),
    //             Action::ClickedInputPort { port_key } => self.project.graph_editor.clicked_input_port( &port_key ),
    //             Action::SetInputPortValue { port_key, display_value } => self.project.graph_editor.set_input_port_value_if_display_value_can_convert(&port_key, &display_value),
    //             Action::ClickedOutputPort { port_key } => self.project.graph_editor.clicked_output_port( &port_key ),
    //             Action::StopPortSearch => self.project.graph_editor.stop_port_search(),
    //             Action::CreateViewport { name } => { self.layout.add_viewport( name ); },
    //             Action::ToggleDebugWindow => 
    //             {
    //                 // self.layout.debug_window_active = !self.layout.debug_window_active
    //             },
    //             Action::StartNodeGraphExecution =>
    //             {
    //                 // let mut empower_executor = EmpowerExecutor::new(self.project.graph_editor.node_graph.clone(), true, true);
    //                 // empower_executor.start_node_graph();

    //                 // self.project.graph_editor.executor = Some( empower_executor );
    //             },
    //             Action::StartNodeGraphExecutionFromEntry { node_key } =>
    //             {
    //                 // let mut empower_executor = EmpowerExecutor::new(self.project.graph_editor.node_graph.clone(), true, true);
    //                 // empower_executor.start_node_graph_from_entry( &node_key );

    //                 // self.project.graph_editor.executor = Some( empower_executor );
    //             },
    //             Action::StopNodeGraphExecution =>
    //             {
    //                 {
    //                     let executor = self.project.graph_editor.executor.as_ref().unwrap(); // @TODO, find a better way to achieve this behavior
    //                     self.project.graph_editor.executor_history = Some( ( executor.start_time.clone(), executor.history.clone() ));
    //                 }
    //                 self.project.graph_editor.executor = None;
    //             },
    //             Action::ToggleExecutionHisotryWindow => 
    //             {
    //                 // self.layout.execution_history_window_active = !self.layout.execution_history_window_active
    //             },
    //             Action::ImportAsset { path } =>
    //             {
    //                 self.project.import_asset(&path);
    //             },
    //             Action::CreateFile { path } =>
    //             {
    //                 self.project.create_file(&path);
    //             },
    //             Action::CreateFolder { path } =>
    //             {
    //                 self.project.create_folder(&path);
    //             }
    //             Action::RenameFile { original_path, new_path } =>
    //             {
    //                 // self.project.rename_file(original_path, new_path);
    //             },
    //             Action::DefaultLayout =>
    //             {
    //                 self.layout = Layout::default_layout();
    //             },
    //             Action::ClearLayout =>
    //             {
    //                 self.layout = Layout::new();
    //             },
    //             Action::SaveEditorState =>
    //             {
    //                 self.layout.save();
    //             },
    //             Action::LoadEditorState =>
    //             {
    //                 self.layout = Layout::load();
    //             },
    //             Action::BeginDraggingAsset { path } =>
    //             {
    //                 // self.layout.dragged_asset = Some( path );
    //             },
    //             Action::StopDraggingAsset =>
    //             {
    //                 // self.layout.dragged_asset = None;
    //             },
    //         }
