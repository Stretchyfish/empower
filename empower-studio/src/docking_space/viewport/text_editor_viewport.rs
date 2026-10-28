use empower_engine::assets::{AssetId, AssetKind, Assets};
use serde::{Deserialize, Serialize};

use crate::studio_context::StudioContext;

#[derive(Clone, Serialize, Deserialize)]
pub struct TextEditorViewport
{
    active_asset: Option<AssetId>,
}

impl TextEditorViewport
{
    pub fn new() -> Self
    {
        Self
        {
            active_asset: None,
        }
    }

    pub fn with_asset(asset_id: AssetId) -> Self
    {
        Self
        {
            active_asset: Some(asset_id),
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, studio_context: &mut StudioContext, viewport_title: &String)
    {
        egui::Panel::top(format!("hotbar_{}", viewport_title)).show_inside(ui, |ui|
        {
            text_editor_hotbar(ui, &mut self.active_asset, &studio_context.get_project().assets, viewport_title);
        });

        text_editor(ui, self.active_asset, studio_context);
    }
}

fn text_editor_hotbar(ui: &mut egui::Ui, active_asset_id: &mut Option<AssetId>, assets: &Assets, viewport_title: &str)
{
    ui.horizontal(|ui|
    {
        ui.label("Asset");

        let mut selected_name = "None";
        if let Some(id) = active_asset_id
        {
            if let Some(meta) = assets.meta.get(id)
            {
                selected_name = &meta.name;
            }
        }

        egui::ComboBox::from_id_salt((viewport_title, "asset_selector"))
            .selected_text(selected_name)
            .show_ui(ui, |ui|
            {
                ui.selectable_value(active_asset_id, None, "None");

                for (id, meta) in &assets.meta
                {
                    if meta.kind == AssetKind::Json
                    {
                        ui.selectable_value(active_asset_id, Some(*id), &meta.name);
                    }
                }
            });
    });

    ui.add_space(2.0);
}

fn text_editor(ui: &mut egui::Ui, active_asset_id: Option<AssetId>, studio_context: &mut StudioContext)
{
    let Some(asset_id) = active_asset_id else
    {
        ui.label("Select a JSON asset to edit.");
        return;
    };

    let assets = &mut studio_context.get_project_mut().assets;
    let Some(text) = assets.loaded_assets.loaded_json.get_mut(&asset_id) else
    {
        ui.label("The selected JSON asset is not loaded.");
        return;
    };

    let response = ui.add_sized(
        ui.available_size(),
        egui::TextEdit::multiline(text).code_editor(),
    );

    if response.changed()
    {
        assets.modified_assets.insert(asset_id);
    }
}

