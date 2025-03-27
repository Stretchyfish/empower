use egui;
use crate::viewports::graph_viewport::GraphViewportState;

pub fn visualize_background(ui: &mut egui::Ui, graph_viewport_state: &GraphViewportState)
{
    let background_rect = ui.max_rect();
    let background_dots_spacing = 20.0 * graph_viewport_state.pan_zoom.zoom_scale;
    // let background_dots_spacing = 20.0;

    let draw_offset = graph_viewport_state.pan_zoom.pan_offset;

    let number_of_cols = (background_rect.width() / background_dots_spacing).ceil() as usize;
    let number_of_rows = (background_rect.height() / background_dots_spacing).ceil() as usize;

    for i in 0..number_of_cols 
    {
        for j in 0..number_of_rows
        {
            let x = (background_rect.left()) + i as f32 * background_dots_spacing + draw_offset.x;
            let y = (background_rect.top()) + j as f32 * background_dots_spacing + draw_offset.y;

            ui.painter().circle_filled(egui::pos2(x, y), 2.0, egui::Color32::BLACK);
        }
    }

    // @TODO, update this way of drawing the dots to work with changing rect sizes
}
