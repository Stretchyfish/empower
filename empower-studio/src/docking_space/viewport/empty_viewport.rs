use crate::studio_context::StudioContext;

use super::Viewport;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
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

    fn clone_box(&self) -> Box<dyn Viewport>
    {
        Box::new( self.clone() )
    }

    fn name(&self) -> &'static str {
        "empty viewport"
    }

    fn show(&mut self, _: &mut egui::Ui, _: &mut StudioContext, _: &String) {
    }
}
