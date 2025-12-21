use empower_engine::runtime::EmpowerExecutor;

use crate::graph_editor::GraphEditor;
use crate::workspace::Layout; 
use crate::actions::Action;

pub struct StudioContext
{
    pub graph_editor: GraphEditor,
    pub layout: Layout,
}

impl StudioContext
{
    pub fn new() -> Self
    {
        Self
        {
            graph_editor: GraphEditor::new(),
            layout: Layout::new(),
        }
    }

    pub fn process_actions(&mut self, action_queue: Vec<Action>)
    {
        for action in action_queue
        {
            match action
            {
                Action::CreateNode { name, position } => { self.graph_editor.add_node(name, position); },
                Action::DeleteNode { node_key } => { self.graph_editor.remove_node(&node_key); },
                Action::CopyNode { node_key } => { todo!() },
                Action::CopySelectedNodes =>
                {
                    for node_key in self.graph_editor.selected_nodes.clone()
                    {
                        self.graph_editor.toggle_node_selection(&node_key);
                        let copied_node_key = self.graph_editor.create_node_copy(&node_key);
                        self.graph_editor.toggle_node_selection(&copied_node_key);
                    }
                },
                Action::ToggleNodeSelection { node_key } => self.graph_editor.toggle_node_selection(&node_key), 
                Action::AddNodeToSelectedNodes { node_key } => self.graph_editor.add_node_to_selection(&node_key),
                Action::AddNodesToSelectedNodes { node_keys } => for node_key in node_keys { self.graph_editor.add_node_to_selection(&node_key); },
                Action::ClearAllNodesFromSelectedNodes => self.graph_editor.clear_node_selection(), // @TOOD, this should be renamed clear?
                Action::MoveSelectedNodes { canvas_delta_position } => self.graph_editor.move_selected_nodes(&canvas_delta_position),
                Action::ClickedInputPort { port_key } => self.graph_editor.clicked_input_port( &port_key ),
                Action::ClickedOutputPort { port_key } => self.graph_editor.clicked_output_port( &port_key ),
                Action::StopPortSearch => self.graph_editor.stop_port_search(),
                Action::CreateViewport { name } => { self.layout.add_viewport( name ); },
                Action::ToggleDebugWindow => self.layout.debug_window_active = !self.layout.debug_window_active,
                Action::StartNodeGraphExecution =>
                {
                    let mut empower_executor = EmpowerExecutor::new(self.graph_editor.node_graph.clone(), true, true);
                    empower_executor.start_node_graph();

                    self.graph_editor.executor = Some( empower_executor );
                },
                Action::StartNodeGraphExecutionFromEntry { node_key } =>
                {
                    let mut empower_executor = EmpowerExecutor::new(self.graph_editor.node_graph.clone(), true, true);
                    empower_executor.start_node_graph_from_entry( &node_key );

                    self.graph_editor.executor = Some( empower_executor );
                },
                Action::StopNodeGraphExecution => self.graph_editor.executor = None,
                Action::None => todo!(),
            }
        }
        
    }
}
