use empower_engine::assets::AssetId;

use crate::studio_context::StudioContext;

pub fn show(image_asset_id: &AssetId, ui: &mut egui::Ui, studio_context: &mut StudioContext, _: &String)
{
    let project = studio_context.get_project_mut();
    let image = project.assets.loaded_assets.loaded_images.get(image_asset_id);

    if image.is_none()
    {
        ui.label("unable to find image");
        return;
    }

    let image_texture = ui.load_texture("image", image.unwrap().clone(), Default::default());

    ui.image(&image_texture);
}
