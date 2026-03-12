use crate::studio_context::project::Project;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct ProjectSettings
{
    show: bool,
}

impl ProjectSettings
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

    pub fn show(&mut self, ctx: &egui::Context, project: &mut Project)
    {
        if !self.show { return; }

        egui::Window::new("Project Settings")
        .collapsible(true)
        .resizable(true)
        .auto_sized()
        .open(&mut self.show)
        .show(ctx, |ui| 
        {

            ui.horizontal(|ui|
            {
                ui.label("Project name");
                ui.text_edit_singleline(&mut project.name);
            });


            ui.label("Location");

            let mut path_string = project.location.clone().to_str().unwrap().to_string();
            let text_edit_widget = egui::TextEdit::singleline( &mut path_string ) // @TODO, look into this again
            .interactive(false);

            ui.add(text_edit_widget);


        
        
        });
    }
}
