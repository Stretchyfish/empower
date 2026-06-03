use crate::studio_context::StudioContext;

pub fn show(ui: &mut egui::Ui, studio_context: &mut StudioContext)
{
    if true // @TODO, replace this with is running
    {
        ui.scope(|ui|
        {
            ui.style_mut().visuals.widgets.inactive.weak_bg_fill = egui::Color32::DARK_GREEN;
            ui.style_mut().visuals.widgets.hovered.weak_bg_fill = egui::Color32::LIGHT_GREEN;
            if ui.button("▶ Start").clicked()
            {

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

            }
            ui.spinner(); 
        });
    }
}
