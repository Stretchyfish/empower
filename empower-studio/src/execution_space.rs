use crate::studio_context::StudioContext;

pub fn show(ui: &mut egui::Ui, studio_context: &mut StudioContext)
{
    let executor = studio_context.get_executor_mut();

    if executor.is_none()
    {
        return;
    }

    let executor = executor.as_mut().unwrap();

    if !executor.is_running()
    {
        studio_context.request_stop_execute();
        return;
    }

    executor.run(Some( ui ));
}
