use crate::{studio_context::StudioContext, user_inputs::UserInputs};

use super::Viewport;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct TerminalViewport
{
    #[serde(skip)]
    index_after_last_observed_output_value: Option<usize>,

    #[serde(skip)]
    entries: Vec<String>,
}

#[typetag::serde]
impl Viewport for TerminalViewport
{
    fn new() -> Box<dyn Viewport>
        where Self:Sized
    {

        Box::new( Self {
            index_after_last_observed_output_value: None,
            entries: Vec::new(),
        })
    }

    fn clone_box(&self) -> Box<dyn Viewport>  {
        Box::new( self.clone() )
    }

    fn name(&self) ->  &'static str {
        "terminal viewport"
    }

    fn show(&mut self, ui: &mut egui::Ui, studio_context: &mut StudioContext, viewport_name: &String, user_inputs: &UserInputs)
    {
        self.extract_new_outputs(studio_context);

        ui.horizontal_top(|ui|
        {
            if ui.button("Clear").clicked()
            {
                self.entries.clear();
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
            for text in &self.entries
            {
                ui.label(text); 
            }
        });
    }
}

impl TerminalViewport
{
    fn extract_new_outputs(&mut self, studio_context: &StudioContext)
    {
        let executor = studio_context.get_executor();

        if executor.is_none()
        {
            return;
        }

        if executor.as_ref().unwrap().settings.outputs.is_none()
        {
            return;
        }

        let outputs = executor.as_ref().unwrap().settings.outputs.as_ref().unwrap();

        if let Some( index_after_last_observed_output_value ) = self.index_after_last_observed_output_value // This is to handle edgecase when same setup is executed twice
        {
            if outputs.len() < index_after_last_observed_output_value
            {
                self.index_after_last_observed_output_value = None;
            }
        }

        if outputs.is_empty()
        {
            return;
        }

        let index_after_last_observed_output_value = self.index_after_last_observed_output_value.unwrap_or(0);

        let new_outputs = &outputs[index_after_last_observed_output_value..];
        self.entries.extend_from_slice(new_outputs);

        self.index_after_last_observed_output_value = Some( outputs.len() );
    }
}
