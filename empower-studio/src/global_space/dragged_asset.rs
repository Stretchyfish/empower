use crate::studio_context::StudioContext;


const DRAGGED_ASSET_TEXT_FONT_SIZE: f32 = 14.0;
const DRAGGED_ASSET_TEXT_BUFFER: f32 = 30.0;

pub fn show(ui: &mut egui::Ui, studio_context: &mut StudioContext)
{
    let dragged_asset = studio_context.get_dragged_asset();

    if dragged_asset.is_none()
    {
        return;
    }

   let asset_name = studio_context.get_project().assets.meta.get(&dragged_asset.unwrap()).unwrap().name.clone(); 

    let mouse_position = ui.pointer_latest_pos().unwrap_or_default();

    let dragged_asset_painter = ui.layer_painter(egui::LayerId::new(
        egui::Order::Foreground,
        egui::Id::new("drag_preview"),
    ));

    let text_size = dragged_asset_painter.layout_no_wrap(
                                            asset_name.to_string(),
                                            egui::FontId::proportional(DRAGGED_ASSET_TEXT_FONT_SIZE),
                                            egui::Color32::YELLOW,
                                        ).size();

    let dragged_asset_rect = egui::Rect::from_center_size(mouse_position, egui::Vec2 { x: text_size.x + DRAGGED_ASSET_TEXT_BUFFER, y: text_size.y + DRAGGED_ASSET_TEXT_BUFFER});

    dragged_asset_painter.rect_filled(dragged_asset_rect, 4.0, egui::Color32::from_gray(40));
    dragged_asset_painter.text(
        dragged_asset_rect.center(),
        egui::Align2::CENTER_CENTER,
        asset_name,
        egui::FontId::proportional(DRAGGED_ASSET_TEXT_FONT_SIZE),
        egui::Color32::WHITE,
    );
}
