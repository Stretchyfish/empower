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

            }
        });
    });
}