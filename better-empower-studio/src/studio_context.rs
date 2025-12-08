use better_empower_engine::runtime::EmpowerExecutor;

use crate::graph_editor::GraphEditor;
use crate::workspace::Layout; 

pub struct StudioContext
{
    pub graph_editor: GraphEditor,
    pub layout: Layout,
    pub executor: Option<EmpowerExecutor>,
}

impl StudioContext
{
    pub fn new() -> Self
    {
        Self
        {
            graph_editor: GraphEditor::new(),
            layout: Layout::new(),
            executor: None,
        }
    }
}