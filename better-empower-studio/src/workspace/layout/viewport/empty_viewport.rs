use crate::GraphEditor;

use super::Viewport;

pub struct EmptyViewport
{

}

impl Viewport for EmptyViewport
{
    fn new() -> Box<dyn Viewport> 
    where
        Self: Sized {

        Box::new( Self {} )
        
    }

    fn name(&self) -> &'static str {
        "empty viewport"
    }

    fn show(&mut self, _: &mut egui::Ui, _: &mut GraphEditor) {
    }
}