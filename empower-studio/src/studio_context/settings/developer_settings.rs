use crate::studio_context::{StudioContext, project::GraphEditor};

#[derive(Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct DeveloperSettings
{
    show: bool,
    pub show_ids: bool,
}

impl DeveloperSettings
{
    pub fn toggle_show(&mut self)
    {
        self.show = !self.show;
    }

    
    pub fn show(&mut self, ctx: &egui::Context, studio_context: &mut StudioContext)
    {
        if !self.show { return; }

        egui::Window::new("Developer Settings")
        .collapsible(true)
        .resizable(true)
        .auto_sized()
        .open(&mut self.show)
        .show(ctx, |ui| 
        {
            ui.horizontal(|ui|
            {
                ui.checkbox(&mut self.show_ids, "show ids");
            });

            ui.collapsing("Project", |_|
            {
                
            });

            ui.collapsing("Settings", |_|
            {
                
            });

            ui.collapsing("Graph", |ui|
            {
                show_graph_state(ui, &studio_context.get_project_mut().graph_editor);
            });

            ui.collapsing("Executor", |_|
            {
                
            });
        });
    }
}

pub fn show_graph_state(ui: &mut egui::Ui, graph_editor: &GraphEditor)
{
    ui.collapsing(format!("selected node {}", graph_editor.selected_nodes.len()), |_|
    {
        
    });

    ui.horizontal(|ui|
    {
        ui.label(format!("port searching: {}", graph_editor.port_searcher.is_some()));


        
    });
    
    ui.horizontal(|ui|
    {
        ui.vertical(|ui|
        {
            ui.heading("Frontend");

            ui.collapsing(format!("display nodes: {}", graph_editor.display_nodes.len()), |ui|
            {
                for (key, display_node) in &graph_editor.display_nodes
                {
                    ui.label("Name, Key");
                    ui.label(&display_node.title);
                    ui.label(&key.to_string());
                }
            
            });
        });

        ui.separator();

        ui.vertical(|ui|
        {
            ui.heading("Backend");
            
            let nodes = graph_editor.node_graph.get_all_nodes();
            ui.collapsing(format!("nodes: {}", nodes.len()), |ui|
            {
                for node in nodes
                {
                    ui.label("Name, Key");
                    ui.label(node.kind.name());
                    ui.label(node.key.to_string());
                }
            
            });
        });

        
    });
}
