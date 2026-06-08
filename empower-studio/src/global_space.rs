use crate::{studio_context::StudioContext, user_inputs::UserInputs};

mod developer_panel;
pub use developer_panel::DeveloperPanel;

pub fn show(ui: &mut egui::Ui, studio_context: &mut StudioContext)
{
    let (windows, user_state) = studio_context.get_windows_mut_and_borrow_user_state();

    // Each window show function does itself keep track of it should have an open window or not!
    windows.developer_panel.show(ui, user_state);

}
