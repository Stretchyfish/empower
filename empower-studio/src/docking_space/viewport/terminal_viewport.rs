use crate::{studio_context::StudioContext, user_inputs::UserInputs};

use super::Viewport;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct TerminalViewport
{
    
}

#[typetag::serde]
impl Viewport for TerminalViewport
{
    fn new() -> Box<dyn Viewport>
        where Self:Sized
    {

        Box::new( Self {} )
    }

    fn clone_box(&self) -> Box<dyn Viewport>  {
        Box::new( self.clone() )
    }

    fn name(&self) ->  &'static str {
        "terminal viewport"
    }

    fn show(&mut self, ui: &mut egui::Ui, studio_context: &mut StudioContext, viewport_name: &String, user_inputs: &UserInputs)
    {
        ui.horizontal_top(|ui|
        {
            if ui.button("Clear").clicked()
            {

            }

            if ui.button("Add text").clicked()
            {

            }

            if ui.button("Add line").clicked()
            {

            }
        });

        egui::ScrollArea::vertical()
        .id_salt(egui::Id::from(self.name()))
        .auto_shrink(false)
        .stick_to_bottom(true)
        .show(ui, |ui|
        {
            // for entry in &logs.entries
            // {
            //     let entry_text = format!("{}", entry.text);
            //     ui.label(entry_text); 
            // }
        });


    }
}
