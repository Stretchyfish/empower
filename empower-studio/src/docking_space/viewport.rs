use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::{studio_context::StudioContext, user_inputs::UserInputs};

mod empty_viewport;
use empty_viewport::EmptyViewport;

mod graph_viewport;
use graph_viewport::GraphEditorViewport;

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
    let mut r: HashMap<&'static str, ViewportConstructor> = HashMap::new();

    r.insert(EmptyViewport::new().name(), || EmptyViewport::new());
    r.insert(GraphEditorViewport::new().name(), || GraphEditorViewport::new());

    r
});
