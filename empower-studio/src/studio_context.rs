use crate::graph_editor::GraphEditor;
use crate::workspace::Workspace;

pub struct StudioContext
{
    pub graph_editor: GraphEditor, // @TODO, determine if they all needs to be public
    pub workspace: Workspace,
}

impl StudioContext
{
    pub fn new() -> Self
    {
        Self
        {
            graph_editor: GraphEditor::new(),
            workspace: Workspace::new(),
        }
    }
}
