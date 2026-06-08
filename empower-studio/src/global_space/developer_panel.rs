use crate::user_state::{self, UserState};


pub struct DeveloperPanel
{
    show: bool,
    
}

impl DeveloperPanel
{
    pub fn new() -> Self
    {
        Self
        {
            show: false,
            
        }
    }

    pub fn toggle_show(&mut self)
    {
        self.show = !self.show;
    }

    pub fn show(&mut self, ui: &mut egui::Ui, user_state: &Option<UserState>)
    {
        if !self.show 
        {
            return;
        }

        egui::Window::new("Developer Panel")
        .collapsible(true)
        .resizable(true)
        .auto_sized()
        .open(&mut self.show)
        .show(ui, |ui| 
        {
            ui.horizontal(|ui|
            {
                ui.label(format!("User state: {:?}", user_state))
            });
        });


        
        
    }
}
