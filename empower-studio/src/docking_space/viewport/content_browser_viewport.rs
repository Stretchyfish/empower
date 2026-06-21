use crate::{studio_context::StudioContext, user_inputs::UserInputs};
use super::Viewport;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct ContentBrowserViewport
{
    
}

#[typetag::serde]
impl Viewport for ContentBrowserViewport
{
    fn new() -> Box<dyn Viewport>where Self:Sized {

        Box::new( Self {} )
    }

    fn clone_box(&self) -> Box<dyn Viewport>  {

        Box::new( self.clone() )
    }

    fn name(&self) ->  &'static str {
        "content browser viewport"
    }

    fn show(&mut self,ui: &mut egui::Ui,studio_context: &mut StudioContext,viewport_name: &String,user_inputs: &UserInputs) {
    }
}
