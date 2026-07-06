use crate::studio_context::StudioContext;

#[derive(Clone)]
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

    pub fn show(&mut self, ui: &mut egui::Ui, studio_context: &mut StudioContext)
    {
        if !self.show 
        {
            return;
        }

        let (settings, assets) = studio_context.get_settings_mut_and_assets();

        egui::Window::new("Developer Panel")
        .collapsible(true)
        .resizable(true)
        .auto_sized()
        .open(&mut self.show)
        .show(ui, |ui| 
        {
            // ui.horizontal(|ui|
            // {
            //     ui.label(format!("User state: {:?}", user_state))
            // });

            ui.collapsing("node graph", |_|
            {
                
            });

            ui.collapsing("assets", |ui|
            {
                ui.collapsing("meta", |ui|
                {
                    egui::Grid::new("assets_meta_visualization")
                    .num_columns(2)
                    .spacing([12.0, 4.0])
                    .striped(true)
                    .show(ui, |ui|
                    {
                        for (asset_id, meta) in &assets.meta
                        {
                            ui.label(asset_id.to_string());
                            ui.label(meta.to_string());
                            ui.end_row();
                        }
                    });
                });

                ui.collapsing("path to id", |ui|
                {
                    egui::Grid::new("assets_path_to_id_visualization")
                    .num_columns(2)
                    .spacing([12.0, 4.0])
                    .striped(true)
                    .show(ui, |ui|
                    {
                        for (path, asset_id) in &assets.path_to_asset_id
                        {
                            ui.label(path.to_string_lossy());
                            ui.label(asset_id.to_string());
                            ui.end_row();
                        }
                    });
                });

                ui.collapsing("node graphs", |ui|
                {
                    egui::Grid::new("assets_node_graph_visualization")
                    .num_columns(2)
                    .spacing([12.0, 4.0])
                    .striped(true)
                    .show(ui, |ui|
                    {
                        for (asset_id, node_graph) in &assets.loaded_node_graphs
                        {
                            ui.label(asset_id.to_string());
                            ui.label(node_graph.name.to_string());
                            ui.end_row();
                        }
                    });
                });
            });

            if ui.checkbox(&mut settings.developer_mode, "developer mode").clicked()
            {
            
            }
        });
    }
}
