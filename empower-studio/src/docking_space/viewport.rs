use std::collections::HashMap;
use once_cell::sync::Lazy;

use crate::{docking_space::viewport::variable_editor_viewport::VariableEditorViewport, studio_context::StudioContext, user_inputs::UserInputs};

mod empty_viewport;
use empty_viewport::EmptyViewport;

mod graph_viewport;
use graph_viewport::GraphViewport;

mod terminal_viewport;
use terminal_viewport::TerminalViewport;

mod content_browser_viewport;
use content_browser_viewport::ContentBrowserViewport;

mod variable_editor_viewport;

#[typetag::serde(tag="viewport_name")]
pub trait Viewport
{
    fn new() -> Box<dyn Viewport> 
    where
        Self: Sized;

    fn clone_box(&self) -> Box<dyn Viewport>;
    fn name(&self) -> &'static str;
    fn show(&mut self, ui: &mut egui::Ui, studio_context: &mut StudioContext, viewport_name: &String, user_inputs: &UserInputs);
}

impl Clone for Box<dyn Viewport>
{
    fn clone(&self) -> Self
    {
        self.clone_box()
    }
}

type ViewportConstructor = fn() -> Box<dyn Viewport>;

pub static VIEWPORT_REGISTRY: Lazy<HashMap<&'static str, ViewportConstructor>> = Lazy::new(|| {
    let mut m: HashMap<&'static str, ViewportConstructor> = HashMap::new();

    m.insert(EmptyViewport::new().name(), || EmptyViewport::new());
    m.insert(GraphViewport::new().name(), || GraphViewport::new());
    m.insert(TerminalViewport::new().name(), || TerminalViewport::new());
    m.insert(ContentBrowserViewport::new().name(), || ContentBrowserViewport::new());
    m.insert(VariableEditorViewport::new().name(), || VariableEditorViewport::new());
 
    m
});

