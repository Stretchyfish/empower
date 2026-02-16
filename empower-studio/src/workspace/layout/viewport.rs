use std::collections::HashMap;
use once_cell::sync::Lazy;

use crate::{GraphEditor, actions::Action, project::Project};

mod empty_viewport;
use empty_viewport::EmptyViewport;

mod graph_viewport;
use graph_viewport::GraphViewport;

mod terminal_viewport;
use terminal_viewport::TerminalViewport;

mod content_browser_viewport;
use content_browser_viewport::ContentBrowserViewport;

use super::DraggedAsset;

#[typetag::serde(tag="viewport_name")]
pub trait Viewport
{
    fn new() -> Box<dyn Viewport> 
    where
        Self: Sized;

    fn name(&self) -> &'static str;
    fn show(&mut self, ui: &mut egui::Ui, project: &mut Project, viewport_name: &String, action_queue: &mut Vec<Action>); // @TODO, should get changed to take project as input
}

type ViewportConstructor = fn() -> Box<dyn Viewport>;

pub static VIEWPORT_REGISTRY: Lazy<HashMap<&'static str, ViewportConstructor>> = Lazy::new(|| {
    let mut m: HashMap<&'static str, ViewportConstructor> = HashMap::new();

    m.insert(EmptyViewport::new().name(), || EmptyViewport::new());
    m.insert(GraphViewport::new().name(), || GraphViewport::new());
    m.insert(TerminalViewport::new().name(), || TerminalViewport::new());
    m.insert(ContentBrowserViewport::new().name(), || ContentBrowserViewport::new());
 
    m
});

