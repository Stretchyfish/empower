use crate::{GraphEditor, actions::Action, project::Project};

use super::Viewport;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct EmptyViewport
{

}

#[typetag::serde]
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

    fn show(&mut self, _: &mut egui::Ui, _: &mut Project, _: &String, _: &mut Vec<Action>) {
    }
}
