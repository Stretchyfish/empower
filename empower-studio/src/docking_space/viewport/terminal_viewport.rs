use empower_engine::utility::log_buffer::LogBuffer;

use crate::studio_context::{StudioContext};

use super::Viewport;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct TerminalViewport
{

}

#[typetag::serde]
impl Viewport for TerminalViewport
{
    fn new() -> Box<dyn Viewport> 
    where
        Self: Sized {
        

        Box::new(
            Self
            {

            }
        )
    }

    fn clone_box(&self) -> Box<dyn Viewport>
    {
        Box::new( self.clone() )
    }

    fn name(&self) -> &'static str {
        "terminal viewport"
    }

    fn show(&mut self, ui: &mut egui::Ui, studio_context: &mut StudioContext, _: &String) {

        let logs = studio_context.get_execution_log();
        
        self.show_text_in_buffer(logs, ui);
    }
}

impl TerminalViewport
{
    fn show_text_in_buffer(&mut self, logs: &LogBuffer, ui: &mut egui::Ui)
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
            for entry in &logs.entries
            {
                let entry_text = format!("{}", entry.text);
                ui.label(entry_text); 
            }
        });
    }
}
