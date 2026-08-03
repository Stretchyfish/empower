use crate::{studio_context::StudioContext, user_inputs::UserInputs};

use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct TerminalViewport
{
    #[serde(skip)]
    index_after_last_observed_output_value: Option<usize>,

    #[serde(skip)]
    entries: Vec<String>,
}

impl TerminalViewport
{
    pub fn new() -> Self
    {
        Self
        {
            index_after_last_observed_output_value: None,
            entries: Vec::new(),
        }
    }
}

pub fn show(terminal_viewport: &mut TerminalViewport, ui: &mut egui::Ui, studio_context: &mut StudioContext, viewport_name: &String, _: &UserInputs)
{
    let developer_mode = studio_context.get_settings().developer_mode;
    
    terminal_viewport.extract_new_outputs(studio_context);

    ui.horizontal_top(|ui|
    {
        if ui.button("Clear").clicked()
        {
            terminal_viewport.entries.clear();
        }

        if ui.button("Add text").clicked()
        {

        }

        if ui.button("Add line").clicked()
        {

        }
    });

    ui.separator();

    egui::ScrollArea::vertical()
    .id_salt(egui::Id::from(viewport_name.clone())) 
    .auto_shrink(false)
    .stick_to_bottom(true)
    .show(ui, |ui|
    {
        for text in &terminal_viewport.entries
        {
            ui.label(text);
        }
    });

    if developer_mode
    {
        show_terminal_viewport_debug_info(terminal_viewport, ui);
    }
}

impl TerminalViewport
{
    fn extract_new_outputs(&mut self, studio_context: &StudioContext)
    {
        let cache = studio_context.get_cache();

        if let Some( index_after_last_observed_output_value ) = self.index_after_last_observed_output_value // This is to handle edgecase when same setup is executed twice
        {
            if cache.session.outputs.len() < index_after_last_observed_output_value
            {
                self.index_after_last_observed_output_value = None;
            }
        }

        if cache.session.outputs.is_empty()
        {
            return;
        }

        let index_after_last_observed_output_value = self.index_after_last_observed_output_value.unwrap_or(0);

        let new_outputs = &cache.session.outputs[index_after_last_observed_output_value..];
        self.entries.extend_from_slice(new_outputs);

        self.index_after_last_observed_output_value = Some( cache.session.outputs.len() );
    }
}

fn show_terminal_viewport_debug_info(terminal_viewport: &mut TerminalViewport, ui: &mut egui::Ui)
{
    let debug_layer = egui::LayerId::new(
        egui::Order::Foreground,
        egui::Id::new("terminal_viewport_debug_info"), // @TODO, this id needs to be unique to viewport
    );

    let viewport_rect = ui.max_rect();

    let painter = ui.layer_painter(debug_layer)
                    .with_clip_rect(viewport_rect);

    let debug_info = format!(
        "\
        Terminal Viewport
        index_after_last_observed_output_value: {:?}
        entries: (only showing length) {:?}"
        , terminal_viewport.index_after_last_observed_output_value
        , terminal_viewport.entries.len(),
    );

    painter.text(
        viewport_rect.left_top() + egui::vec2(10.0, 10.0),
        egui::Align2::LEFT_TOP,
        debug_info,
        egui::FontId::monospace(14.0),
        egui::Color32::RED,
    );
}
