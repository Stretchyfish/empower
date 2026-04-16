use std::path::PathBuf;

use crate::{studio_context::StudioContext, user_inputs::UserInputs};

pub fn show(ctx: &egui::Context, studio_context: &mut StudioContext,user_inputs: &UserInputs)
{
    process_global_user_inputs(studio_context, user_inputs);

    show_dragged_asset(ctx, studio_context.get_dragged_asset(), user_inputs);
    show_windows(ctx, studio_context, user_inputs);
    show_settings_windows(ctx, studio_context);

    show_execution(ctx, studio_context);
}

fn process_global_user_inputs(studio_context: &mut StudioContext, user_inputs: &UserInputs)
{
    if user_inputs.holding_alt && user_inputs.clicked_d
    {
        studio_context.get_settings_mut().developer_settings.toggle_show();
    }

    if user_inputs.holding_ctrl && user_inputs.clicked_s
    {
        if studio_context.project_is_temporary()
        {
            studio_context.request_save_project_as();
            return;
        }
        studio_context.request_save_project();
    }
}

fn show_dragged_asset(ctx: &egui::Context, path: &Option<PathBuf>, user_inputs: &UserInputs)
{
    if path.is_none()
    {
        return;
    }

    let asset_name = path.as_ref().unwrap().file_name().unwrap().to_string_lossy().to_string();

    let dragged_asset_painter = ctx.layer_painter(egui::LayerId::new(
            // egui::Order::Foreground,
            egui::Order::Tooltip,
            egui::Id::new("drag_preview"),
    ));
    
    let text_font_size = 14.0; // @TODO, make these global
    let text_buffer = 30.0;

    let text_size = dragged_asset_painter.layout_no_wrap(
                                                asset_name.to_string(),
                                                egui::FontId::proportional(text_font_size),
                                                egui::Color32::YELLOW,
                                            ).size();

    let dragged_asset_rect = egui::Rect::from_center_size(user_inputs.mouse_position, egui::Vec2 { x: text_size.x + text_buffer, y: text_size.y + text_buffer });
    dragged_asset_painter.rect_filled(dragged_asset_rect, 4.0, egui::Color32::from_gray(40));
    dragged_asset_painter.text(
        dragged_asset_rect.center(),
        egui::Align2::CENTER_CENTER,
        asset_name,
        egui::FontId::proportional(text_font_size),
        egui::Color32::WHITE,
    );
}

fn show_windows(ctx: &egui::Context, studio_context: &mut StudioContext, user_inputs: &UserInputs) // @TODO, find a better name for this!
{
    let mut windows = studio_context.get_windows_clone();

    windows.project_name_window.show(ctx, studio_context, user_inputs);

    studio_context.set_windows(windows);
}

fn show_settings_windows(ctx: &egui::Context, studio_context: &mut StudioContext)
{
    let mut settings = studio_context.get_settings_clone(); // @TODO, consider combining a getter function for get project and settings to avoid this clone
    settings.project_settings.show(ctx, studio_context.get_project_mut());
    settings.developer_settings.show(ctx, studio_context);
    settings.export_settings.show(ctx, studio_context.get_project_mut());

    studio_context.set_settings(settings);
}

fn show_execution(ctx: &egui::Context, studio_context: &mut StudioContext)
{
    if !studio_context.exeucutor_is_running()
    {
        return;
    }

    studio_context.execute_node_graph(ctx);
}
