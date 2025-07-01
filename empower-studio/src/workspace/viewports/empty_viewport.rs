pub struct EmptyViewport
{
    pub title: String,
}

impl EmptyViewport
{
   pub fn new(title: String) -> Self
   {
        Self 
        {
            title,
        }
   }
}

pub fn show(ui: &mut egui::Ui)
{

}