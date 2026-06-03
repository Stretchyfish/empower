use crate::studio_context::StudioContext;

pub fn show(ui: &mut egui::Ui, studio_context: &mut StudioContext)
{
    ui.label("Start graph");

    let project = studio_context.get_project_mut();

    ui.menu_button(project.entry_graph.to_string(), |_|
    {

    });
}
