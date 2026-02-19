use chrono::format::StrftimeItems;

use crate::actions::Action;

pub fn show_execution_history_button(ui: &mut egui::Ui, action_queue: &mut Vec<Action>)
{
    // let studio_has_execution_history = studio_context.project.graph_editor.executor_history.is_some();
    let studio_has_execution_history = true;
    
    if studio_has_execution_history
    {
        // let execution_history = studio_context.project.graph_editor.executor_history.as_ref().unwrap();

        // let time_as_text = format!("{}", execution_history.0.format("%Y-%m-%d %H:%M:%S"));
        // ui.label(time_as_text);

        if ui.button("show execution history").clicked()
        {
            action_queue.push( Action::ToggleExecutionHisotryWindow );
        }
    }
}
