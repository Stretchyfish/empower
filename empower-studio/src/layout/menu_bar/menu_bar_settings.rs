use crate::actions::Action;

pub fn show_menu_bar_settings(ui: &mut egui::Ui, action_queue: &mut Vec<Action>)
{
    if ui.button("Toggle debug window").clicked()
    {
        action_queue.push( Action::ToggleDebugWindow );
    }
}
