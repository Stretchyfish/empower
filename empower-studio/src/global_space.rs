use chrono::format::strftime;

use crate::{studio_context::{Settings, StudioContext}, user_inputs::UserInputs};


pub fn show(ctx: &egui::Context, studio_context: &mut StudioContext,user_inputs: &UserInputs)
{
    process_global_user_inputs(studio_context, user_inputs);
    show_windows(ctx, studio_context, user_inputs);
    show_settings_windows(ctx, studio_context);
}

fn process_global_user_inputs(studio_context: &mut StudioContext, user_inputs: &UserInputs)
{
    if user_inputs.holding_alt && user_inputs.clicked_d
    {
        studio_context.get_settings_mut().developer_settings.toggle_show();
    }

    if user_inputs.holding_ctrl && user_inputs.clicked_s
    {
        studio_context.request_save_project();
    }
}

fn show_windows(ctx: &egui::Context, studio_context: &mut StudioContext, user_inputs: &UserInputs) // @TODO, find a better name for this!
{
    let mut windows = studio_context.get_windows_clone();

    windows.project_name_window.show(ctx, studio_context, user_inputs);

    studio_context.set_windows(windows);
}

fn show_settings_windows(ctx: &egui::Context, studio_context: &mut StudioContext)
{
    let mut settings = studio_context.get_settings_clone();
    settings.project_settings.show(ctx, studio_context.get_project_mut());
    settings.developer_settings.show(ctx, studio_context);

    studio_context.set_settings(settings);
}
