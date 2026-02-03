use std::{collections::HashMap, path::PathBuf};

use crate::{actions::Action, project::{Asset, AssetId, AssetKind, Project, ProjectState}};

use super::Viewport;

const THUMBNAIL_SIZE: egui::Vec2 = egui::Vec2 { x: 100.0, y: 100.0 };
const ELEMENT_SPACING: f32 = 10.0;

pub struct ContentBrowserViewport
{
    current_directory: PathBuf,
    selected_asset: Option<AssetId>,
}


impl Viewport for ContentBrowserViewport
{
    fn new() -> Box<dyn Viewport> 
    where
        Self: Sized {

        Box::new( Self { current_directory: PathBuf::new(), selected_asset: None } ) // @TODO, set this up properly!
    }

    fn name(&self) -> &'static str {
        "content browser viewport"
    }

    fn show(&mut self, ui: &mut egui::Ui, project: &mut Project, viewport_name: &String, action_queue: &mut Vec<Action>) {

        if ui.button("import asset").clicked()
        {
            if project.state == ProjectState::Temporary
            {
                action_queue.push(Action::SaveProject);
                return;
            }
            
            let file_path = rfd::FileDialog::new() // @TODO, consider if this should be in the project struct instead of the viewport?
                                                .set_title("Import asset") // @TODO, this should probably be in the action of import asset
                                                .pick_file();

            if file_path.is_none()
            {
                panic!("Failed to get file path in import asset!"); // @TODO, in the future, handle this error properly!
            }

            action_queue.push( Action::ImportAsset { path: file_path.unwrap() });
        }

        ui.separator();

        egui::panel::SidePanel::left("something").show_inside(ui, |ui|
        {
            ui.label("test");
        });

        egui::panel::CentralPanel::default().show_inside(ui, |ui|
        {
            
                egui::ScrollArea::vertical().show(ui, |ui|
                {
                    let available_width = ui.available_width();
                    let max_elements_per_row = ((available_width + ELEMENT_SPACING) / (THUMBNAIL_SIZE.x + ELEMENT_SPACING)).floor() as i32;

                    self.show_content_browser_elements(ui, project, max_elements_per_row);
                });
        });

    }
}

impl ContentBrowserViewport
{
    fn show_content_browser_elements(&mut self, ui: &mut egui::Ui, project: &Project, max_elements_per_row: i32)
    {
        let mut element_index = 0;

        let asset_keys: Vec<_> = project.assets.keys().collect();

        // @TODO, all the code below here needs a rework
        
        let mut keep_showing_elements = true;
        while keep_showing_elements
        {
            ui.horizontal(|ui|
            {
                let mut elements_in_row = 0;
                while elements_in_row < max_elements_per_row
                {
                    if element_index >= asset_keys.len()
                    {
                        keep_showing_elements = false;
                        break;
                    }
                    
                    let asset = project.assets.get(asset_keys[element_index]).unwrap(); 

                    let icon = String::from("📃");

                    let mut is_asset_selected = false;
                    if self.selected_asset.is_some()
                    {
                        is_asset_selected = self.selected_asset.unwrap() == asset.id;
                    }
                    
                    if ui.selectable_label(is_asset_selected, icon).clicked()
                    {
                        self.selected_asset = Some( asset.id );

                        if is_asset_selected
                        {
                            self.selected_asset = None;
                        }
                    }

                    elements_in_row += 1;
                    element_index += 1;
                }
            });
        }
    }
}
