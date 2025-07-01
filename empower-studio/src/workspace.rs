mod manu_bar;
pub mod viewports;
mod tab_viewer;
use tab_viewer::TabsViewer;
use viewports::Viewports;
use viewports::viewport_type::ViewportType;

use crate::studio_context::StudioContext;

pub struct Workspace
{
    docking_state: egui_dock::DockState<String>,
    viewports: Viewports,
}

impl Workspace
{
   pub fn new() -> Self
   {
        let mut new_workspace = Self 
        { 
            docking_state: egui_dock::DockState::new(Vec::new()), 
            viewports: Viewports::new(), 
        };

        let startup_viewport_type = ViewportType::GraphViewport;
        new_workspace.add_viewport(startup_viewport_type);

        new_workspace
   } 

   pub fn add_viewport(&mut self, viewport_type: ViewportType) // @TODO, look into deletion of tabs, and naming of newly generated tabs after deletion
   {
        let new_viewport_name = self.viewports.add_viewport(viewport_type);
        self.docking_state.push_to_focused_leaf(new_viewport_name);
   }
}

pub fn show(ctx: &egui::Context, studio_context: &mut StudioContext)
{
    egui::TopBottomPanel::top("menu bar").show(ctx, |ui| 
    {
        manu_bar::show(ui, studio_context);
    });

    egui::CentralPanel::default()
        .frame(egui::Frame::central_panel(&ctx.style()).inner_margin(0.))
        .show(ctx, |ui| 
    {
        egui_dock::DockArea::new(&mut studio_context.workspace.docking_state)
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
                &mut TabsViewer {
                    graph_editor: &mut studio_context.graph_editor,
                    viewports: &mut studio_context.workspace.viewports,
                },
            );
     });
}