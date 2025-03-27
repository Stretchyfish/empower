use egui;

#[derive(Default)]
pub struct EmptyViewport
{
    pub title: String,
}

impl EmptyViewport
{
    pub fn new(initial_title: String) -> Self
    {
        Self 
        {  
            title: initial_title,
        }
    }

    pub fn view(&mut self, _: &mut egui::Ui)
    {
        // egui::Frame::group(ui.style())        
        // .inner_margin(0.0)
        // .show(ui, |ui|
        // {
        //     ui.painter().circle(egui::Pos2 { x: 100.0, y: 100.0 }, 50.0, egui::Color32::YELLOW, egui::Stroke::NONE);
        // });

        // egui::ScrollArea::both()
        //     .enable_scrolling(true)
        //     .hscroll(true)
        //     .vscroll(true)
        //     .show(ui, |ui|
        // {
        //     let position = egui::Pos2 { x: 2000.0, y: 500.0};
        //     ui.painter().circle(position, 50.0, egui::Color32::RED, Stroke::NONE);


        //     ui.add_sized([100., 200.], egui::Button::new("Second"));
        // });
    }
}
