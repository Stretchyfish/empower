use crate::actions::Action;

pub fn show_menu_bar_windows(ui: &mut egui::Ui, action_queue: &mut Vec<Action>) // @TODO, not a great name
{
    ui.menu_button("Windows", |ui|
    {
        if ui.button("default layout").clicked()
        {
            action_queue.push( Action::DefaultLayout );
        }
        if ui.button("clear layout").clicked()
        {
            action_queue.push( Action::ClearLayout );
        }
        if ui.button("Graph viewport").clicked()
        {
            action_queue.push( Action::CreateViewport { name: "graph viewport" });
        }
        if ui.button("Terminal Viewport").clicked()
        {
            action_queue.push( Action::CreateViewport { name: "terminal viewport" });
        }
        if ui.button("Content Browser Viewport").clicked()
        {
            action_queue.push( Action::CreateViewport { name: "content browser viewport" });
        }
        if ui.button("Empty Viewport").clicked()
        {
            action_queue.push( Action::CreateViewport { name: "empty viewport" });
        }

        if ui.button("save editor state").clicked()
        {
            action_queue.push( Action::SaveEditorState );
        }

        if ui.button("load editor state").clicked()
        {
            action_queue.push( Action::LoadEditorState );
        }
    });
}
