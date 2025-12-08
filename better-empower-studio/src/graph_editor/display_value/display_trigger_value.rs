use super::DisplayValueKind;

pub struct DisplayTriggerValue
{

}

impl DisplayValueKind for DisplayTriggerValue
{
    fn text(&self) -> String {
        todo!()
    }

    fn get_color(&self) -> egui::Color32 {
        todo!()
    }
    
    fn show(&mut self, ui: &mut egui::Ui, input_port_position: &egui::Pos2) {

        let port_value_size = egui::Vec2{ x: 120.0, y: 0.0 };
        let port_value_rect = egui::Rect::from_min_size(input_port_position.clone(), port_value_size);

        let mut boolean = true;

        let checkbox = egui::Checkbox::new(
                                               &mut boolean, 
                                                    egui::RichText::new("").font(egui::FontId::proportional(35.0)));
        ui.put(port_value_rect, checkbox);
    }
}