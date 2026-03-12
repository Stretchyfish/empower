use std::collections::HashMap;

use crate::{studio_context::StudioContext, user_inputs::UserInputs};

pub mod viewport;
use viewport::Viewport;

pub fn show(ctx: &egui::Context, studio_context: &mut StudioContext, user_inputs: &UserInputs)
{
    let mut layout_mut = studio_context.get_layout_clone(); // This is a rather expensive call, but needed to maintain structure

    egui::CentralPanel::default()
    .frame(egui::Frame::central_panel(&ctx.style()).inner_margin(0.0))
    .show(ctx, |ui| 
    {
        egui_dock::DockArea::new(&mut layout_mut.docking_state)
            .style({
                let mut style = egui_dock::Style::from_egui(ctx.style().as_ref());
                style.tab_bar.fill_tab_bar = true;
                style
            })
            .show_close_buttons(true) // @TODO, add behavior here?
            .show_add_popup(true)
            .show_leaf_close_all_buttons(false)
            .show_leaf_collapse_buttons(false)
            .show_inside(
                ui,
                &mut TabViewer {
                    studio_context,
                    viewports: &mut layout_mut.viewports,
                    user_inputs,
                },
            );
    });

    studio_context.set_layout(layout_mut); // Since all other layout modifications are done through request, this should be safe
}

pub struct TabViewer<'a>
{
    pub studio_context: &'a mut StudioContext,
    pub viewports: &'a mut HashMap<String, Box<dyn Viewport>>,
    pub user_inputs: &'a UserInputs,
}

impl egui_dock::TabViewer for TabViewer<'_>
{
    type Tab = String;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText
    {
        tab.clone().into()  // VERY IMPORTANT, that the titles are unique (Current implementation will have problems with this!)
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) 
    {
        let tab_name: String = tab.clone();

        if !self.viewports.contains_key(&tab_name)
        {
            panic!("Requested a viewport not in the viewport that doesn't exist");
        }

        let viewport = self.viewports.get_mut(&tab_name).unwrap();
        viewport.show(ui, self.studio_context, &tab_name, self.user_inputs);
    }
}
