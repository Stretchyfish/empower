use std::collections::BTreeMap;

use empower_engine::assets::{AssetId, AssetKind};

use crate::studio_context::StudioContext;

pub fn show(ui: &mut egui::Ui, studio_context: &mut StudioContext)
{
    ui.label("Start graph");

    let project = studio_context.get_project_mut();

    let mut new_start_node_graph_id = None;

    {
        
        let sorted_names: BTreeMap<&AssetId, String> = project.assets.meta.iter().filter(|(_, m)| m.kind == AssetKind::NodeGraph ).map(|(k, m)| (k, m.name.clone()) ).collect();
        let current_start_graph_name = project.assets.meta.get(&project.entry_graph).unwrap().name.clone();

        egui::ComboBox::from_id_salt("start graph selector") 
        .selected_text( &current_start_graph_name )
        .show_ui(ui, |ui|
        {
            for (id, text) in sorted_names 
            {
                if id == &project.entry_graph
                {
                    continue;
                }
        
                ui.selectable_value( &mut new_start_node_graph_id, Some( *id ), text);
            }
        });
    }

    if new_start_node_graph_id.is_some()
    {
        project.entry_graph = new_start_node_graph_id.unwrap();
    }
}
