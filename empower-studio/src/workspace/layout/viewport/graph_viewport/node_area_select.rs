use empower_engine::NodeGraphKey;

#[derive(Clone)]
pub struct NodeAreaSelect
{
    pub start_point: egui::Pos2,
    pub rect: egui::Rect,
    pub nodes_inside_rect: Vec<NodeGraphKey>,
}

impl NodeAreaSelect
{
    pub fn determine_area_select_rect(&mut self, mouse_position: &egui::Pos2)
    {
        // Draw different kinds of rectangles depending on mouse positions

        if mouse_position.x > self.start_point.x
        {
            self.rect = egui::Rect::from_two_pos(*mouse_position, self.start_point);

            return;
        }

        self.rect = egui::Rect::from_two_pos(self.start_point, *mouse_position);
    }
}
