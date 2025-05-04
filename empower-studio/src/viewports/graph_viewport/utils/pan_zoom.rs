use crate::interactions;

pub struct PanZoom
{
    pub zoom_scale: f32,
    pub pan_offset: egui::Vec2,
    pub window_size_pan_offset: egui::Vec2,
    zoom_speed: f32,
}

impl Default for PanZoom
{
    fn default() -> Self
    {
        Self
        {
            zoom_scale: 1.0,
            pan_offset: egui::Vec2 { x: 0.0, y: 0.0 },
            window_size_pan_offset: egui::Vec2 { x: 0.0, y: 0.0 },
            zoom_speed: 0.01,
        }
    }
}

impl PanZoom
{
    pub fn new() -> Self
    {
        Self
        {
            zoom_scale: 1.0,
            pan_offset: egui::Vec2 { x: 0.0, y: 0.0 },
            window_size_pan_offset: egui::Vec2 { x: 0.0, y: 0.0 },
            zoom_speed: 0.001,
        }
    }

    pub fn update_pan(&mut self, user_input: &interactions::user::UserInputs)
    {
        self.pan_offset += user_input.mouse_position_delta / self.zoom_scale;
    }

    pub fn update_zoom(&mut self, user_input: &interactions::user::UserInputs)
    {
        let mouse_position_world_space_before_zoom = self.screen_to_world(&user_input.mouse_position);

        self.zoom_scale += user_input.scroll_delta * self.zoom_speed;
        self.zoom_scale = self.zoom_scale.clamp(0.1, 10.0);
        
        let mouse_position_world_space_after_zoom = self.screen_to_world(&user_input.mouse_position);

        self.pan_offset -= mouse_position_world_space_before_zoom - mouse_position_world_space_after_zoom;
    }
    
    pub fn world_to_screen(&self, world_position: &egui::Pos2) -> egui::Pos2
    {
        (*world_position + self.pan_offset) * self.zoom_scale
    }

    pub fn screen_to_world(&self, screen_position: &egui::Pos2) -> egui::Pos2
    {
        // Should never happen, but make sure to add a divide by 0 check here
        // *screen_position / self.zoom_scale + self.pan_offset
        *screen_position / self.zoom_scale - self.pan_offset
    }
}
