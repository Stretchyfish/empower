use super::Viewport;

const THUMBNAIL_SIZE: egui::Vec2 = egui::Vec2 { x: 100.0, y: 100.0 };
const ELEMENT_SPACING: f32 = 10.0;

pub struct ContentBrowserViewport
{
    
}


impl Viewport for ContentBrowserViewport
{
    fn new() -> Box<dyn Viewport> 
    where
        Self: Sized {

        Box::new( Self {} )
    }

    fn name(&self) -> &'static str {
        "content browser viewport"
    }

    fn show(&mut self, ui: &mut egui::Ui, graph_editor: &mut crate::graph_editor::GraphEditor, viewport_name: &String, action_queue: &mut Vec<crate::actions::Action>) {

        egui::ScrollArea::vertical().show(ui, |ui|
        {
            let available_width = ui.available_width();
            let items_per_row = ((available_width + ELEMENT_SPACING) / (THUMBNAIL_SIZE.x + ELEMENT_SPACING));

        });
    }
}
