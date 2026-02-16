use crate::{actions::Action, studio_context::StudioContext};

use super::menu_bar;
use super::debug_window;
use super::history_window;
use super::project_settings_window;
use super::project_name_window;

pub fn show(ctx: &egui::Context, studio_context: &mut StudioContext, action_queue: &mut Vec<Action>)
{
    debug_window::show(ctx, studio_context);
    history_window::show(ctx, studio_context);
    project_settings_window::show(ctx, studio_context);
    project_name_window::show(ctx, studio_context, action_queue);

    if studio_context.layout.dragged_asset.is_some()
    {
        let asset_name = studio_context.layout.dragged_asset.as_ref().unwrap().file_name().unwrap().to_string_lossy().to_string();

        let mouse_position = ctx.pointer_latest_pos().unwrap_or_default();
    
        let dragged_asset_painter = ctx.layer_painter(egui::LayerId::new(
            egui::Order::Foreground,
            egui::Id::new("drag_preview"),
        ));

        let text_font_size = 14.0; // @TODO, make these global
        let text_buffer = 30.0;
        
        let text_size = dragged_asset_painter.layout_no_wrap(
                                                asset_name.to_string(),
                                                egui::FontId::proportional(text_font_size),
                                                egui::Color32::YELLOW,
                                            ).size();

        let dragged_asset_rect = egui::Rect::from_center_size(mouse_position, egui::Vec2 { x: text_size.x + text_buffer, y: text_size.y + text_buffer });

        dragged_asset_painter.rect_filled(dragged_asset_rect, 4.0, egui::Color32::from_gray(40));
        dragged_asset_painter.text(
            dragged_asset_rect.center(),
            egui::Align2::CENTER_CENTER,
            asset_name,
            egui::FontId::proportional(text_font_size),
            egui::Color32::WHITE,
        );
    }
}
