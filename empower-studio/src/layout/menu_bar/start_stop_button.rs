use crate::actions::Action;

pub fn show_start_stop_button(ui: &mut egui::Ui, action_queue: &mut Vec<Action>)
{
    // let executor_is_running = studio_context.project.graph_editor.executor.is_none()
    let executor_is_running = false;
    
    if !executor_is_running
    {
        ui.scope(|ui|
        {
            ui.style_mut().visuals.widgets.inactive.weak_bg_fill = egui::Color32::DARK_GREEN;
            ui.style_mut().visuals.widgets.hovered.weak_bg_fill = egui::Color32::LIGHT_GREEN;
            if ui.button("▶ Start").clicked()
            {
                action_queue.push( Action::StartNodeGraphExecution );
                // studio_context.project.graph_editor.executor_history = Some( (Local::now(), TextBuffer::new()) ); // @TODO, make this behavior better
            }
        });
    }
    else 
    {
        ui.scope(|ui|
        {
            ui.style_mut().visuals.widgets.inactive.weak_bg_fill = egui::Color32::DARK_RED;
            ui.style_mut().visuals.widgets.hovered.weak_bg_fill = egui::Color32::LIGHT_RED;
            if ui.button("⏹ Stop").clicked()
            {
                action_queue.push( Action::StopNodeGraphExecution );
                {
                    // let executor = studio_context.project.graph_editor.executor.as_ref().unwrap(); // @TODO, find a better way to achieve this behavior
                    // studio_context.project.graph_editor.executor_history = Some( ( executor.start_time.clone(), executor.history.clone() )); // @TODO, make a shared way to save this independent of where its stopped
                }
            }
            ui.spinner(); 
        });
    }
}
