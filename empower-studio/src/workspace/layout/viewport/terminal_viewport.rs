use empower_engine::utility::text_buffer::TextBuffer;

use crate::{GraphEditor, actions::Action};

use super::Viewport;

pub struct TerminalViewport
{
    text: TextBuffer,
    last_logging_size: Option<usize>, // @TODO, this is not an ideal solution for detecting new values
}

impl Viewport for TerminalViewport
{
    fn new() -> Box<dyn Viewport> 
    where
        Self: Sized {
        

        Box::new(
            Self
            {
                text: TextBuffer::new(),
                last_logging_size: None,
            }
        )
    }

    fn name(&self) -> &'static str {
        "terminal viewport"
    }

    fn show(&mut self, ui: &mut egui::Ui, graph_editor: &GraphEditor, _: &String, _: &mut Vec<Action>) {
        self.show_text_in_buffer(ui);
        self.add_more_text_if_executer_generated_some(graph_editor);
    }

}

impl TerminalViewport
{
    fn show_text_in_buffer(&mut self, ui: &mut egui::Ui)
    {
        ui.horizontal_top(|ui|
        {
            if ui.button("Clear").clicked()
            {
                self.text.clear();
            }

            if ui.button("Add text").clicked()
            {
                // self.add_text(String::from("test something"));
            }

            if ui.button("Add line").clicked()
            {
                // self.add_line(String::from("test something"));
            }
        });

        egui::ScrollArea::vertical()
        .id_salt(egui::Id::from(self.name()))
        .auto_shrink(false)
        .stick_to_bottom(true)
        .show(ui, |ui|
        {
            for line in self.text.lines.iter()
            {
                let line_text = format!("{}", line);
                ui.label(line_text); 
            }
        });
    }

    fn add_more_text_if_executer_generated_some(&mut self, graph_editor: &GraphEditor)
    {
        if graph_editor.executor.is_none()
        {
            self.last_logging_size = None;
            return;
        }

        let log = &graph_editor.executor.as_ref().unwrap().log;

        if self.last_logging_size.is_none()
        {
            self.last_logging_size = Some( 0 );
        }

        let start_looking_index = self.last_logging_size.as_mut().unwrap();

        // This approach breaks down after log hits 1000 lines, its max, will cause problems!
        for i in *start_looking_index..log.lines.len()
        {
            let new_line = log.lines[i].clone();
            self.text.add_line(&new_line);

            // @TODO, this approach does not account for text added to an existing line
        } 

        *start_looking_index = log.lines.len();
    }
}
