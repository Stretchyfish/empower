use egui;

pub struct NewGraphViewport
{
    pub title: String,
    scene_rect: egui::Rect,
}

impl NewGraphViewport
{
    pub fn new(title: String) -> Self
    {
        Self
        {
            title,
            scene_rect: egui::Rect { min: egui::Pos2 { x: -200.0, y: -200.0 }, max: egui::Pos2 { x: 200.0, y: 200.0 }},
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui)
    {
        egui::Scene::new()
        .zoom_range(0.1..=2.0)
        .show(ui, &mut self.scene_rect, |ui|
        {
            let position = egui::Pos2::new(0.0, 0.0);
            ui.painter().circle_filled(position, 20.0, egui::Color32::YELLOW);

            let position_2 = egui::Pos2::new(800.0, 0.0);
            ui.painter().circle_filled(position_2, 20.0, egui::Color32::YELLOW);
        });
    }
}
