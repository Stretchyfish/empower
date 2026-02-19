use crate::{actions::Action, user_inputs::UserInputs, user_state::UserState};

pub fn show_global_space(ctx: &egui::Context, user_state: &UserState, user_inputs: &UserInputs, action_queue: &mut Vec<Action>)
{
    process_global_user_inputs(ctx, user_inputs, action_queue);

}

fn process_global_user_inputs(ctx: &egui::Context, user_inputs: &UserInputs, action_queue: &mut Vec<Action>)
{
    let save_requested = ctx.input(|i| { i.key_pressed(egui::Key::S) && i.modifiers.ctrl});
    if save_requested
    {
        action_queue.push( Action::SaveProject );
    }
}
