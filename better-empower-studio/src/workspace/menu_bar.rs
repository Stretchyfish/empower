use crate::StudioContext;

pub fn show(ui: &mut egui::Ui, studio_context: &mut StudioContext)
{
    egui::MenuBar::new()
    .ui(ui, |ui|
    {
        ui.menu_button("Nodes", |ui|
        {
            if ui.button("Add default node").clicked()
            {
                studio_context.graph_editor.add_node("addition", egui::Pos2 { x: 0.0, y: 0.0 });
            }
        });
    });
}