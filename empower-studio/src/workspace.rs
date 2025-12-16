use egui;

use super::StudioContext;

mod tab_viewer;
use tab_viewer::TabViewer;

pub mod layout;
pub use layout::Layout;

mod menu_bar;
mod debug_window;

pub fn show(ctx: &egui::Context, studio_context: &mut StudioContext)
{

    debug_window::show(ctx, studio_context);

    egui::TopBottomPanel::top("menu bar").show(ctx, |ui| 
    {
        menu_bar::show(ui, studio_context);
    });

    egui::CentralPanel::default()
    .frame(egui::Frame::central_panel(&ctx.style()).inner_margin(0.))
    .show(ctx, |ui| 
    {
        egui_dock::DockArea::new(&mut studio_context.layout.docking_state)
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
                    graph_editor: &mut studio_context.graph_editor,
                    viewports: &mut studio_context.layout.viewports,
                },
            );

        // @TODO, this needs to be moved elsewhere!
        if studio_context.graph_editor.executor.is_some()
        {
            let executor = studio_context.graph_editor.executor.as_mut().unwrap();

            if !executor.is_running()
            {
                studio_context.graph_editor.executor = None;
                return; // @TODO, figure out a better way than returning here
            }

            executor.execute_node_graph(Some( ui ));
        }

        // @TODO, find a better location for this?
        let clicked_backspace = ui.input(|i| i.key_pressed(egui::Key::Backspace));
        if clicked_backspace
        {
            let mut nodes_to_delete = Vec::new();
            for node_key in studio_context.graph_editor.selected_nodes.iter() // @TODO, find a more effecient way of writting this
            {
                nodes_to_delete.push(node_key.clone()); 
            }

            for node_key in nodes_to_delete
            {
                studio_context.graph_editor.remove_node(&node_key);
                studio_context.graph_editor.selected_nodes = Vec::new();
            }
        }
    });
}