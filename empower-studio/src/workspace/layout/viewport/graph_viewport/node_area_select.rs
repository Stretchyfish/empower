use empower_engine::NodeGraphKey;

#[derive(Clone)]
pub struct NodeAreaSelect
{
    pub start_point: egui::Pos2,
    pub rect: egui::Rect,
    pub nodes_inside_rect: Vec<NodeGraphKey>, // @TODO, change this to public once graph viewport is updated
}

impl NodeAreaSelect
{
    pub fn new(start_point: egui::Pos2) -> Self
    {
        Self
        {
            start_point,
            rect: egui::Rect::ZERO,
            nodes_inside_rect: Vec::new(),
        }
    }

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

    pub fn get_nodes_inside_of_area_select(&self) -> Vec<NodeGraphKey>
    {
        self.nodes_inside_rect.clone()
    }

    // @TODO, maybe find a better name?
    pub fn check_if_node_is_inside_area_select_and_add_if_it_is(&mut self, node_key: &NodeGraphKey, node_rect: &egui::Rect)
    {
        if self.rect.contains_rect( *node_rect )
        {
            self.add_node_to_nodes_inside_of_rect( node_key );
        }
    }

    fn add_node_to_nodes_inside_of_rect(&mut self, node_key: &NodeGraphKey)
    {
        if self.nodes_inside_rect.contains(node_key)
        {
            return;
        }

        self.nodes_inside_rect.push( *node_key );
    }
}
