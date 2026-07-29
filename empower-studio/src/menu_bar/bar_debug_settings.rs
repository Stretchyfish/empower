use crate::studio_context::StudioContext;

pub fn show(ui: &mut egui::Ui, studio_context: &mut StudioContext)
{
    {
        let settings = studio_context.get_settings_mut();

        ui.checkbox(&mut settings.debug_mode, "debug mode");

        if !settings.debug_mode
        {
            return;
        }
    }

    let executor_settings = studio_context.get_executor_settings_mut();

    let mut delay = if executor_settings.artificial_delay.is_none()
    {
        0.0
    }
    else
    {
        executor_settings.artificial_delay.unwrap().as_secs_f32()
    };

    ui.add( egui::Slider::new( &mut delay, 0.0..= 10.0 ).text("delay") );

    executor_settings.set_artificial_delay(delay);
}

