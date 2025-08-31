mod manu_bar;
pub mod viewports;
mod tab_viewer;
// mod debug_window;
use tab_viewer::TabsViewer;
use viewports::Viewports;
use viewports::viewport_type::ViewportType;

use crate::studio_context::StudioContext;

pub struct Workspace
{
    docking_state: egui_dock::DockState<String>,
    pub viewports: Viewports, // @TODO, find a way to make these private?
    debug_window_active: bool
}

impl Workspace
{
   pub fn new() -> Self
   {
        let mut new_workspace = Self 
        { 
            docking_state: egui_dock::DockState::new(Vec::new()), 
            viewports: Viewports::new(), 
            debug_window_active: false,
        };

        let graph_viewport_type = ViewportType::GraphViewport;
        let graph_viewport_title = new_workspace.add_viewport(graph_viewport_type);

        let terminal_viewport_type = ViewportType::TerminalViewport;
        let terminal_viewport_title = new_workspace.add_viewport(terminal_viewport_type);

        // This setup is needed to instantiate split docking state (consider in the future to abstract this)
        let graph_viewport_index = new_workspace.docking_state.find_tab(&graph_viewport_title).expect("Unable to find initial graph viewport tab");
        let terminal_viewport_index = new_workspace.docking_state.find_tab(&terminal_viewport_title).expect("Unable to find initial terminal viewport tab");

        new_workspace.docking_state.remove_tab(terminal_viewport_index).expect("Terminal viewport missing from docking state");
        new_workspace.docking_state.main_surface_mut().split_below(graph_viewport_index.1, 0.7, vec![terminal_viewport_title]);

        new_workspace
   } 

   pub fn add_viewport(&mut self, viewport_type: ViewportType) -> String // @TODO, look into deletion of tabs, and naming of newly generated tabs after deletion
   {
        let new_viewport_name = self.viewports.add_viewport(viewport_type);
        self.docking_state.push_to_focused_leaf(new_viewport_name.clone());

        new_viewport_name
   }
}

pub fn show(ctx: &egui::Context, studio_context: &mut StudioContext)
{
    // debug_window::show(ctx, studio_context);

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

        // let clicked_backspace = ui.input(|i| i.key_pressed(egui::Key::Backspace));
        // if clicked_backspace
        // {
        //     let mut nodes_to_delete = Vec::new();
        //     for node_key in studio_context.graph_editor.selected_nodes.iter() // @TODO, find a more effecient way of writting this
        //     {
        //         nodes_to_delete.push(node_key.clone()); 
        //     }

        //     for node_key in nodes_to_delete
        //     {
        //         studio_context.graph_editor.remove_node(&node_key);
        //         studio_context.graph_editor.selected_nodes = Vec::new();
        //     }
        // }

    });
}