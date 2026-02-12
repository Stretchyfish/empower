use empower_engine::runtime::EmpowerExecutor;
use crate::graph_editor::GraphEditor;
use crate::workspace::Layout; 
use crate::actions::Action;
use crate::project::{Project, ProjectState};

pub struct StudioContext
{
    pub project: Project,
    pub layout: Layout,
}

impl StudioContext
{
    pub fn new() -> Self
    {
        Self
        {
            project: Project::new(),
            layout: Layout::new(),
        }
    }

    pub fn process_actions(&mut self, action_queue: Vec<Action>, ctx: &egui::Context)
    {
        for action in action_queue
        {
            match action
            {
                Action::CreateTemporaryProject =>
                {
                    self.project.new_project();
                },
                Action::SaveProject => 
                { 
                    match self.project.state
                    {
                        ProjectState::Undefined => todo!(),
                        ProjectState::Temporary(_) => 
                        {
                            self.layout.project_name_window = Some( self.project.name.clone() );
                            return;
                        },
                        ProjectState::Saved(_) => self.project.save(),
                    }

                    ctx.send_viewport_cmd(egui::ViewportCommand::Title(format!("empower studio - {}", self.project.name)));
                },
                Action::SaveProjectAs =>
                {
                    match self.project.state
                    {
                        ProjectState::Undefined => todo!(),
                        ProjectState::Temporary(_) => 
                        {
                            self.layout.project_name_window = Some( self.project.name.clone() );
                            return;
                        },
                        ProjectState::Saved(_) => self.project.save_as(),
                    }
                    ctx.send_viewport_cmd(egui::ViewportCommand::Title(format!("empower studio - {}", self.project.name)));
                },
                Action::LoadProject =>
                {
                    self.project.load();
                    ctx.send_viewport_cmd(egui::ViewportCommand::Title(format!("empower studio - {}", self.project.name)));
                },
                Action::CreateNode { name, position } => { self.project.graph_editor.add_node(name, position); },
                Action::RefreshNodeStructure { node_key } => { self.project.graph_editor.refresh_node_strcuture(&node_key); },
                Action::DeleteNode { node_key } => { self.project.graph_editor.remove_node(&node_key); },
                Action::CopySelectedNodes =>
                            {
                                for node_key in self.project.graph_editor.selected_nodes.clone()
                                {
                                    self.project.graph_editor.toggle_node_selection(&node_key);
                                    let copied_node_key = self.project.graph_editor.create_node_copy(&node_key);
                                    self.project.graph_editor.toggle_node_selection(&copied_node_key);
                                }
                            },
                Action::ToggleNodeSelection { node_key } => self.project.graph_editor.toggle_node_selection(&node_key),
                Action::AddNodesToSelectedNodes { node_keys } => for node_key in node_keys { self.project.graph_editor.add_node_to_selection(&node_key); },
                Action::ClearAllNodesFromSelectedNodes => self.project.graph_editor.clear_node_selection(),
                Action::MoveSelectedNodes { canvas_delta_position } => self.project.graph_editor.move_selected_nodes(&canvas_delta_position),
                Action::ClickedInputPort { port_key } => self.project.graph_editor.clicked_input_port( &port_key ),
                Action::SetInputPortValue { port_key, display_value } => self.project.graph_editor.set_input_port_value_if_display_value_can_convert(&port_key, &display_value),
                Action::ClickedOutputPort { port_key } => self.project.graph_editor.clicked_output_port( &port_key ),
                Action::StopPortSearch => self.project.graph_editor.stop_port_search(),
                Action::CreateViewport { name } => { self.layout.add_viewport( name ); },
                Action::ToggleDebugWindow => self.layout.debug_window_active = !self.layout.debug_window_active,
                Action::StartNodeGraphExecution =>
                            {
                                let mut empower_executor = EmpowerExecutor::new(self.project.graph_editor.node_graph.clone(), true, true);
                                empower_executor.start_node_graph();

                                self.project.graph_editor.executor = Some( empower_executor );
                            },
                Action::StartNodeGraphExecutionFromEntry { node_key } =>
                            {
                                let mut empower_executor = EmpowerExecutor::new(self.project.graph_editor.node_graph.clone(), true, true);
                                empower_executor.start_node_graph_from_entry( &node_key );

                                self.project.graph_editor.executor = Some( empower_executor );
                            },
                Action::StopNodeGraphExecution =>
                {
                    {
                        let executor = self.project.graph_editor.executor.as_ref().unwrap(); // @TODO, find a better way to achieve this behavior
                        self.project.graph_editor.executor_history = Some( ( executor.start_time.clone(), executor.history.clone() ));
                    }
                    self.project.graph_editor.executor = None;
                },
                Action::ToggleExecutionHisotryWindow => self.layout.execution_history_window_active = !self.layout.execution_history_window_active,
                Action::ImportAsset { path } =>
                {
                    self.project.import_asset(&path);
                },
                Action::CreateFile { path } =>
                {
                    self.project.create_file(&path);
                },
                Action::CreateFolder { path } =>
                {
                    self.project.create_folder(&path);
                }
                Action::RenameFile { original_path, new_path } =>
                {
                    self.project.rename_file(original_path, new_path);
                }

            }
        }
        
    }
}
