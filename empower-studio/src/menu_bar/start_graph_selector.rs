use std::collections::BTreeMap;

use empower_engine::assets::AssetId;

use crate::studio_context::StudioContext;

pub fn show(ui: &mut egui::Ui, studio_context: &mut StudioContext)
{
    ui.label("Start graph");

    let project = studio_context.get_project_mut();

    let graph_names = project.assets.get_all_node_graph_names(); // @TODO, this is currently unsorted
    let sorted_names: BTreeMap<AssetId, String> = graph_names.into_iter().collect();

    let selected_graph_name = match project.assets.load_node_graph(&project.location, &project.entry_graph )
    {
        Some( node_graph ) => node_graph.name.clone(),
        None =>
        {
            println!("Menu bar was asked to get an graph asset id that doesn't exist in assets");
            "unknown".to_string()   
        },
    };


    ui.menu_button(selected_graph_name, |ui|
    {
        for (_, text) in sorted_names
        {
            ui.label(text);
        }
    });
}
