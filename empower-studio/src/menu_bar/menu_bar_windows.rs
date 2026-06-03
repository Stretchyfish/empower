use crate::studio_context::StudioContext;

pub fn show(ui: &mut egui::Ui, studio_context: &mut StudioContext)
{
    ui.menu_button("Windows", |ui|
    {
        if ui.button("default layout").clicked()
        {
            studio_context.request_default_layout();
        }
        ui.menu_button("Add window", |ui|
        {
            if ui.button("Graph viewport").clicked()
            {
                studio_context.request_new_viewport("graph viewport");
            }
            if ui.button("Terminal Viewport").clicked()
            {
                studio_context.request_new_viewport("terminal viewport");
            }
            if ui.button("Content Browser Viewport").clicked()
            {
                studio_context.request_new_viewport("content browser viewport");
            }
            if ui.button("Empty Viewport").clicked()
            {
                studio_context.request_new_viewport("empty viewport");
            }
            if ui.button("Variable Editor Viewport").clicked()
            {
                studio_context.request_new_viewport("variable editor viewport");
            }
            if ui.button("Timeline Viewport").clicked()
            {
                studio_context.request_new_viewport("timeline viewport");
            }
        });

        if ui.button("save layout").clicked()
        {
            studio_context.request_save_studio();
        }

        if ui.button("load layout").clicked()
        {
            studio_context.request_load_studio();
        }
    });
}
