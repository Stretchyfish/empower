
pub struct Settings
{
    pub windows: Windows,
}

impl Settings
{
    pub fn new() -> Self
    {
        Self
        {
            windows: Windows::new(),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Windows
{
    pub developer_settings: DeveloperSettings,

}

impl Windows
{
    pub fn new() -> Self
    {
        Self
        {
            developer_settings: DeveloperSettings::default(),
            
        }
    }
}

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct DeveloperSettings
{
    pub show: bool,
}

impl DeveloperSettings
{
    pub fn show(&mut self, ctx: &egui::Context)
    {
        egui::Window::new("Developer settings")
        .collapsible(true)
        .resizable(true)
        .auto_sized()
        .open(&mut self.show)
        .show(ctx, |ui| 
        {



            
            
        });
    }
}
