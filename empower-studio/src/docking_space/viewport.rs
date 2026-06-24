use std::collections::HashMap;
use serde::{Serialize, Deserialize};

use once_cell::sync::Lazy;

use crate::{studio_context::StudioContext, user_inputs::UserInputs};

pub mod empty_viewport;
pub use empty_viewport::EmptyViewport;

pub mod graph_viewport;
pub use graph_viewport::GraphViewport;

pub mod terminal_viewport;
pub use terminal_viewport::TerminalViewport;

pub mod content_browser_viewport;
pub use content_browser_viewport::ContentBrowserViewport;

#[derive(Clone, Serialize, Deserialize)]
pub enum Viewport
{
    Graph { graph_viewport: GraphViewport },
    Terminal { terminal_viewport: TerminalViewport },
    ContentBrowser { content_browser_viewport: ContentBrowserViewport },
    Empty { empty_viewport: EmptyViewport },
    
}

// #[typetag::serde(tag="viewport_name")]
// pub trait Viewport
// {
//     fn new() -> Box<dyn Viewport> 
//     where
//         Self: Sized;

//     fn clone_box(&self) -> Box<dyn Viewport>;
//     fn name(&self) -> &'static str;
//     fn show(&mut self, ui: &mut egui::Ui, studio_context: &mut StudioContext, viewport_name: &String, user_inputs: &UserInputs);
// }

// impl Clone for Box<dyn Viewport>
// {
//     fn clone(&self) -> Self
//     {
//         self.clone_box()
//     }
// }

// type ViewportConstructor = fn() -> Box<dyn Viewport>;

// pub static VIEWPORT_REGISTRY: Lazy<HashMap<&'static str, ViewportConstructor>> = Lazy::new(|| {
//     let mut r: HashMap<&'static str, ViewportConstructor> = HashMap::new();

//     r.insert(EmptyViewport::new().name(), || EmptyViewport::new());
//     r.insert(GraphEditorViewport::new().name(), || GraphEditorViewport::new());
//     r.insert(TerminalViewport::new().name(), || TerminalViewport::new());
//     r.insert(ContentBrowserViewport::new().name(), || ContentBrowserViewport::new());

//     r
// });
