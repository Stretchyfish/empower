use super::DisplayNodeKind;

#[derive(Clone)]
pub struct DefaultDisplayNode
{

}

impl DisplayNodeKind for DefaultDisplayNode
{
    fn new() -> Box<dyn DisplayNodeKind> where
        Self: Sized {
        
        Box::new( Self {} )
    }

    fn clone_box(&self) -> Box<dyn DisplayNodeKind> {
        Box::new( self.clone() )
    }

    fn node_size(&self) -> egui::Vec2 {
        egui::Vec2 { x: 350.0, y: 230.0 }
    }

    fn state_size(&self) -> egui::Vec2 {
        egui::Vec2 { x: 0.0, y: 0.0 }
    }

    fn state_show(&mut self, _: &mut Option<egui::Ui>) -> bool {
        false
    }
}