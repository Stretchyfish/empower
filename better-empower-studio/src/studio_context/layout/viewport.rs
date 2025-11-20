use std::collections::HashMap;
use once_cell::sync::Lazy;

mod empty_viewport;
use empty_viewport::EmptyViewport;

pub trait Viewport
{
    fn new() -> Box<dyn Viewport> 
    where
        Self: Sized;

    fn name(&self) -> &'static str;
    fn show(&mut self, ui: &mut egui::Ui);
}

type ViewportConstructor = fn() -> Box<dyn Viewport>;

pub static VIEWPORT_REGISTRY: Lazy<HashMap<&'static str, ViewportConstructor>> = Lazy::new(|| {
    let mut m: HashMap<&'static str, ViewportConstructor> = HashMap::new();

    m.insert(EmptyViewport::new().name(), || EmptyViewport::new());
 
    m
});

