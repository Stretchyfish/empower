use crate::studio_context::StudioContext;


pub fn show(ctx: &egui::Context, studio_context: &mut StudioContext)
{
    let mut window_active = studio_context.layout.execution_history_window_active;
    egui::Window::new("History Panel")
    .collapsible(true)
    .resizable(false)
    .open(&mut window_active)
    .show(ctx, |ui| 
    {
        if studio_context.graph_editor.executor_history.is_none()
        {
            ui.label("Error, window appeared, but no execution has happened yet");
            return;
        }

        for node_key in &studio_context.graph_editor.executor_history.as_ref().unwrap().1
        {
            ui.label(format!("{}", node_key));
        }

    });

    studio_context.layout.execution_history_window_active = window_active;
}
