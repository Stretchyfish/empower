use empower_engine::EmpowerKey;

pub struct DisplayNode
{
    pub key: EmpowerKey,
    pub position: egui::Pos2,
    pub size: egui::Vec2,
}

impl DisplayNode
{
    pub fn new() -> Self
    {
        Self
        {
            key: 0,
            position: egui::Pos2::new(0.0, 0.0),
            size: egui::Vec2 { x: 200.0, y: 200.0 }
        }
    }

    pub fn new_with_key(new_key: EmpowerKey) -> Self
    {
        Self
        {
            key: new_key,
            position: egui::Pos2::new(0.0, 0.0),
            size: egui::Vec2 { x: 200.0, y: 200.0 }
        }
    }

    pub fn new_with_key_and_position(new_key: EmpowerKey, new_position: egui::Pos2, new_size: egui::Vec2) -> Self
    {
        Self
        {
            key: new_key,
            position: new_position,
            size: new_size        
        }
    }
}

